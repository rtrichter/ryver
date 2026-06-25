use crate::Result;
use crate::object::{FileEntry, Operation, Patch, Tree};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const STORE_DIR: &str = ".ryver";
const OBJECTS_DIR: &str = "objects";
const PATCHES_DIR: &str = "patches";
const REFS_DIR: &str = "refs";
const INDEXES_DIR: &str = "indexes";
const HEAD_REF: &str = "refs/HEAD";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub operation: Operation,
}

#[derive(Debug, Clone)]
pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn init(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();
        let store = root.join(STORE_DIR);

        fs::create_dir_all(store.join(OBJECTS_DIR))?;
        fs::create_dir_all(store.join(PATCHES_DIR))?;
        fs::create_dir_all(store.join(REFS_DIR))?;
        fs::create_dir_all(store.join(INDEXES_DIR))?;

        let config = store.join("config.toml");
        if !config.exists() {
            fs::write(
                config,
                "format_version = 1\nobject_format = \"sha256\"\npatch_format = \"json\"\n",
            )?;
        }

        let head = store.join(HEAD_REF);
        if !head.exists() {
            fs::write(head, "")?;
        }

        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    pub fn discover(start: impl AsRef<Path>) -> Result<Self> {
        let mut current = start.as_ref().canonicalize()?;

        loop {
            if current.join(STORE_DIR).is_dir() {
                return Ok(Self { root: current });
            }

            if !current.pop() {
                return Err("not inside a ryver repository; run `ryver init` first".into());
            }
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn head(&self) -> Result<Option<String>> {
        let value = fs::read_to_string(self.store_path().join(HEAD_REF))?;
        let id = value.trim();

        Ok((!id.is_empty()).then(|| id.to_string()))
    }

    pub fn load_patch(&self, id: &str) -> Result<Patch> {
        let path = self.patch_path(id);
        let bytes = fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub fn head_patch(&self) -> Result<Option<Patch>> {
        self.head()?.map(|id| self.load_patch(&id)).transpose()
    }

    pub fn status(&self) -> Result<Vec<Change>> {
        let base = self
            .head_patch()?
            .map(|patch| patch.tree)
            .unwrap_or_default();
        let current = self.scan_working_tree()?;

        Ok(diff_trees(&base, &current))
    }

    pub fn record(&self, message: String, author: String) -> Result<Patch> {
        let parent = self.head()?;
        let base = match parent.as_deref() {
            Some(id) => self.load_patch(id)?.tree,
            None => Tree::new(),
        };
        let tree = self.scan_working_tree()?;
        let operations = diff_trees(&base, &tree)
            .into_iter()
            .map(|change| change.operation)
            .collect::<Vec<_>>();

        if operations.is_empty() {
            return Err("nothing to record".into());
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let id = patch_id(
            parent.as_deref(),
            &author,
            timestamp,
            &message,
            &operations,
            &tree,
        )?;
        let patch = Patch {
            id,
            parent,
            author,
            timestamp,
            message,
            operations,
            tree,
        };

        self.write_patch(&patch)?;
        fs::write(self.store_path().join(HEAD_REF), format!("{}\n", patch.id))?;

        Ok(patch)
    }

    pub fn log(&self) -> Result<Vec<Patch>> {
        let mut patches = Vec::new();
        let mut current = self.head()?;

        while let Some(id) = current {
            let patch = self.load_patch(&id)?;
            current = patch.parent.clone();
            patches.push(patch);
        }

        Ok(patches)
    }

    fn store_path(&self) -> PathBuf {
        self.root.join(STORE_DIR)
    }

    fn patch_path(&self, id: &str) -> PathBuf {
        self.store_path()
            .join(PATCHES_DIR)
            .join(format!("{id}.json"))
    }

    fn write_patch(&self, patch: &Patch) -> Result<()> {
        let path = self.patch_path(&patch.id);
        let mut file = fs::File::create_new(path)?;
        let json = serde_json::to_vec_pretty(patch)?;
        file.write_all(&json)?;
        file.write_all(b"\n")?;
        Ok(())
    }

    fn scan_working_tree(&self) -> Result<Tree> {
        let mut tree = Tree::new();
        self.scan_dir(&self.root, &mut tree)?;
        Ok(tree)
    }

    fn scan_dir(&self, dir: &Path, tree: &mut Tree) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();

            if should_skip(&file_name) {
                continue;
            }

            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                self.scan_dir(&path, tree)?;
            } else if metadata.is_file() {
                let bytes = fs::read(&path)?;
                let hash = sha256_hex(&bytes);
                let rel = relative_path(&self.root, &path)?;
                self.write_object(&hash, &bytes)?;
                tree.insert(
                    rel,
                    FileEntry {
                        hash,
                        len: metadata.len(),
                    },
                );
            }
        }

        Ok(())
    }

    fn write_object(&self, hash: &str, bytes: &[u8]) -> Result<()> {
        let path = self.store_path().join(OBJECTS_DIR).join(hash);
        if !path.exists() {
            fs::write(path, bytes)?;
        }

        Ok(())
    }
}

pub fn diff_trees(base: &Tree, current: &Tree) -> Vec<Change> {
    let paths = base
        .keys()
        .chain(current.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    paths
        .into_iter()
        .filter_map(|path| match (base.get(&path), current.get(&path)) {
            (None, Some(new)) => Some(Change {
                operation: Operation::Create {
                    path,
                    new: new.clone(),
                },
            }),
            (Some(old), Some(new)) if old != new => Some(Change {
                operation: Operation::Modify {
                    path,
                    old: old.clone(),
                    new: new.clone(),
                },
            }),
            (Some(old), None) => Some(Change {
                operation: Operation::Delete {
                    path,
                    old: old.clone(),
                },
            }),
            _ => None,
        })
        .collect()
}

fn should_skip(file_name: &OsStr) -> bool {
    matches!(file_name.to_str(), Some(STORE_DIR | ".git" | "target"))
}

fn patch_id(
    parent: Option<&str>,
    author: &str,
    timestamp: u64,
    message: &str,
    operations: &[Operation],
    tree: &Tree,
) -> Result<String> {
    let bytes = serde_json::to_vec(&(parent, author, timestamp, message, operations, tree))?;
    Ok(sha256_hex(&bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);

    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }

    output
}

fn relative_path(root: &Path, path: &Path) -> Result<String> {
    let rel = path.strip_prefix(root)?;
    Ok(rel
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn init_creates_store_layout() {
        let dir = tempdir().unwrap();
        Repository::init(dir.path()).unwrap();

        assert!(dir.path().join(".ryver/objects").is_dir());
        assert!(dir.path().join(".ryver/patches").is_dir());
        assert!(dir.path().join(".ryver/refs/HEAD").is_file());
        assert!(dir.path().join(".ryver/config.toml").is_file());
    }

    #[test]
    fn status_reports_added_modified_and_deleted_files() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        fs::write(dir.path().join("a.txt"), "one").unwrap();
        repo.record(
            "first".to_string(),
            "Ryan <github@rtrichter.com>".to_string(),
        )
        .unwrap();

        fs::write(dir.path().join("a.txt"), "two").unwrap();
        fs::write(dir.path().join("b.txt"), "new").unwrap();
        fs::remove_file(dir.path().join("a.txt")).unwrap();

        let changes = repo.status().unwrap();
        assert_eq!(changes.len(), 2);
        assert!(matches!(
            &changes[0].operation,
            Operation::Delete { path, .. } if path == "a.txt"
        ));
        assert!(matches!(
            &changes[1].operation,
            Operation::Create { path, .. } if path == "b.txt"
        ));
    }

    #[test]
    fn record_writes_patch_and_advances_head() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();
        let patch = repo
            .record(
                "initial".to_string(),
                "Ryan <github@rtrichter.com>".to_string(),
            )
            .unwrap();

        assert_eq!(repo.head().unwrap(), Some(patch.id.clone()));
        assert!(
            dir.path()
                .join(format!(".ryver/patches/{}.json", patch.id))
                .is_file()
        );
        assert_eq!(patch.operations.len(), 1);
    }
}

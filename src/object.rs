use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileEntry {
    pub hash: String,
    pub len: u64,
}

pub type Tree = BTreeMap<String, FileEntry>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Operation {
    Create {
        path: String,
        new: FileEntry,
    },
    Modify {
        path: String,
        old: FileEntry,
        new: FileEntry,
    },
    Delete {
        path: String,
        old: FileEntry,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Patch {
    pub id: String,
    pub parent: Option<String>,
    pub author: String,
    pub timestamp: u64,
    pub message: String,
    pub operations: Vec<Operation>,
    pub tree: Tree,
}

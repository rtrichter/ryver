use crate::Result;
use crate::object::Operation;
use crate::repo::{Change, Repository};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

#[derive(Debug, Parser)]
#[command(name = "ryver")]
#[command(about = "A local-first patch-oriented version control experiment")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Status,
    Record {
        #[arg(short, long)]
        message: String,

        #[arg(long)]
        author: Option<String>,
    },
    Log,
    Show {
        id: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path } => {
            let repo = Repository::init(path)?;
            println!("Initialized ryver repository in {}", repo.root().display());
        }
        Command::Status => {
            let repo = Repository::discover(".")?;
            let changes = repo.status()?;

            if changes.is_empty() {
                println!("clean");
            } else {
                for change in changes {
                    println!("{}", format_change(&change));
                }
            }
        }
        Command::Record { message, author } => {
            let repo = Repository::discover(".")?;
            let author = author.unwrap_or_else(default_author);
            let patch = repo.record(message, author)?;
            println!("Recorded patch {}", short_id(&patch.id));
        }
        Command::Log => {
            let repo = Repository::discover(".")?;
            for patch in repo.log()? {
                println!("patch {}", patch.id);
                println!("Author: {}", patch.author);
                println!("Date:   {}", patch.timestamp);
                println!();
                println!("    {}", patch.message);
                println!();
            }
        }
        Command::Show { id } => {
            let repo = Repository::discover(".")?;
            let id = match id {
                Some(id) => id,
                None => repo.head()?.ok_or("no patches recorded yet")?,
            };
            let patch = repo.load_patch(&id)?;

            println!("patch {}", patch.id);
            println!("Author: {}", patch.author);
            println!("Date:   {}", patch.timestamp);
            println!();
            println!("    {}", patch.message);
            println!();

            for operation in &patch.operations {
                println!("{}", format_operation(operation));
            }
        }
    }

    Ok(())
}

fn format_change(change: &Change) -> String {
    format_operation(&change.operation)
}

fn format_operation(operation: &Operation) -> String {
    match operation {
        Operation::Create { path, .. } => format!("A {path}"),
        Operation::Modify { path, .. } => format!("M {path}"),
        Operation::Delete { path, .. } => format!("D {path}"),
    }
}

fn default_author() -> String {
    if let Ok(author) = env::var("RYVER_AUTHOR") {
        return author;
    }

    if let (Ok(name), Ok(email)) = (env::var("GIT_AUTHOR_NAME"), env::var("GIT_AUTHOR_EMAIL")) {
        return format!("{name} <{email}>");
    }

    if let (Some(name), Some(email)) = (git_config("user.name"), git_config("user.email")) {
        return format!("{name} <{email}>");
    }

    "unknown <unknown>".to_string()
}

fn git_config(key: &str) -> Option<String> {
    let output = ProcessCommand::new("git")
        .args(["config", "--get", key])
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
}

fn short_id(id: &str) -> &str {
    id.get(..12).unwrap_or(id)
}

pub mod cli;
pub mod object;
pub mod repo;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

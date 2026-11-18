pub mod attributes;
pub mod binary;
pub mod container;
pub mod database;
pub mod editor;
pub mod extended;
pub mod search;

pub type Result<T> = std::result::Result<T, String>;

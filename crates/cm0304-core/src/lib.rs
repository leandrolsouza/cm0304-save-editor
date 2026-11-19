pub mod attributes;
pub mod binary;
pub mod career;
pub mod container;
pub mod database;
pub mod editor;
pub mod extended;
pub mod health;
pub mod search;
pub mod staff;
pub mod transaction;

pub type Result<T> = std::result::Result<T, String>;

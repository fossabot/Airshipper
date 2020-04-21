//! common client stuff like logging, panic handling, filesystem/networking.

pub mod config;
pub mod error;
pub mod filesystem;
pub mod logger;
pub mod network;
pub mod process;
pub mod profile;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // TODO

// Reexports
pub use tracing::*;

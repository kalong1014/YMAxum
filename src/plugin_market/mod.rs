//! Plugin market module
//! Provides functionality for discovering, installing, updating, and managing plugins

pub mod core;
pub mod models;
pub mod repository;

pub use core::PluginMarket;
pub use models::{PluginInfo, PluginManifest, PluginVersion, SearchOptions};
pub use repository::{PluginRepository, RepositoryError};

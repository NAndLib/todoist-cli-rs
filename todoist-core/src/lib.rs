//! # Todoist Core
//! This create is a library intended to provide a plugable interface into the [Todoist Sync API].
//!
//! This includes:
//! - Type definitions in `type`
//! - Builtin async caching using [Redis]
//! - Async requests to the sync API
//!
//! [Todoist Sync API]: https://developer.todoist.com/sync/v8/#overview
//! [Redis]: https://docs.rs/redis/latest/redis/

/// Core plugin API
pub mod core;
/// Redis caching implementation
pub mod cache;

/// Todoist sync API reqwest client
pub mod client;
/// Todoist sync API type implementation
pub mod types;

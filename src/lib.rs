//! Library root for the `teamy-robocopy` crate.
//!
//! This crate contains the robocopy log parsing functionality and a small CLI
//! (extracted from `teamy-mft`). It also provides logging initialization helpers
//! so consumers can initialize tracing the same way the binary does.

pub mod cli;
pub mod logging;
pub mod robocopy;

/// Re-export the logging initializer so callers can do `teamy_robocopy::init_tracing`.
///
/// The function signature mirrors the helper in `logging.rs` and accepts a
/// `tracing::Level` and a reference to `cli::json_log_behaviour::JsonLogBehaviour`.
pub use crate::logging::init_tracing;

/// Re-export the default JSON log path helper.
pub use crate::logging::default_json_log_path;

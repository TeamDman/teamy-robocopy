use chrono::Local;
use eyre::Result;
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::Level;
use tracing::{debug, info};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Extra filters appended to the default filter when debug/trace is enabled.
///
/// Keeps parity with the original project so logging noise is handled consistently.
pub const DEFAULT_EXTRA_FILTERS: &str = r"bevy_shader=warn,offset_allocator=warn,bevy_app=info,bevy_render=info,gilrs=info,cosmic_text=info,naga=warn,wgpu=error,wgpu_hal=warn,bevy_skein=trace,bevy_winit::system=info";

/// Initialize tracing subscriber with the given log level and optional JSON output.
///
/// This mirrors the behavior from the original project: a human-friendly stderr
/// layer is always registered, and an optional JSON file writer is registered
/// when `json_behaviour` requests an output path.
///
/// # Errors
///
/// Returns an error if directory creation or file access for the JSON log fails.
/// If initializing the global subscriber fails (commonly in test environments
/// where multiple test harnesses attempt to initialize tracing), the function
/// prints a diagnostic to stderr and returns Ok(()) so callers can continue.
pub fn init_tracing(
    level: Level,
    json_behaviour: &crate::cli::json_log_behaviour::JsonLogBehaviour,
) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::builder().parse_lossy(format!(
            "{default_log_level},{extras}",
            default_log_level = level,
            extras = match level {
                Level::DEBUG | Level::TRACE => DEFAULT_EXTRA_FILTERS,
                _ => "",
            }
        ))
    });

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(cfg!(debug_assertions))
        .with_line_number(true)
        .with_writer(std::io::stderr)
        .pretty();

    if let Some(json_log_path) = json_behaviour.get_path() {
        let json_log_path = json_log_path.into_owned();
        if let Some(parent) = json_log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(&json_log_path)?;
        let file = Arc::new(Mutex::new(file));
        let json_writer = {
            let file = Arc::clone(&file);
            BoxMakeWriter::new(move || {
                file.lock()
                    .expect("failed to lock json log file")
                    .try_clone()
                    .expect("failed to clone json log file handle")
            })
        };

        let json_format = tracing_subscriber::fmt::format().json();
        let json_layer = tracing_subscriber::fmt::layer()
            .event_format(json_format)
            .with_file(true)
            .with_target(false)
            .with_line_number(true)
            .with_writer(json_writer);

        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(stderr_layer)
            .with(json_layer);

        if let Err(error) = subscriber.try_init() {
            eprintln!(
                "Failed to initialize tracing subscriber - are you running `cargo test`? If so, multiple test entrypoints may be running from the same process. https://github.com/tokio-rs/console/issues/505 : {error}"
            );
            return Ok(());
        }

        info!(path = %json_log_path.display(), "JSON log output initialized");
    } else {
        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(stderr_layer);
        if let Err(error) = subscriber.try_init() {
            eprintln!(
                "Failed to initialize tracing subscriber - are you running `cargo test`? If so, multiple test entrypoints may be running from the same process. https://github.com/tokio-rs/console/issues/505 : {error}"
            );
            return Ok(());
        }
    }

    debug!("Tracing initialized with level: {:?}", level);
    Ok(())
}

/// Return a default JSON log path when the user requests automatic JSON path selection.
///
/// The format uses a timestamp to avoid collisions: `teamy_robocopy_log_{TIMESTAMP}.jsonl`
#[must_use]
pub fn default_json_log_path() -> PathBuf {
    let timestamp = Local::now().format("%Y-%m-%d_%Hh%Mm%Ss");
    PathBuf::from(format!("teamy_robocopy_log_{timestamp}.jsonl"))
}

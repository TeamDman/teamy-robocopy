pub mod robocopy_logs_tui;

#[allow(
    clippy::module_inception,
    reason = "module structure requires submodule with same name"
)]
mod command;

pub use command::Command;

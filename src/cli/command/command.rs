use crate::cli::command::robocopy_logs_tui::RobocopyLogsTuiArgs;
use crate::cli::global_args::GlobalArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

/// Teamy MFT commands
#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum Command {
    /// Explore robocopy logs in a TUI (validate file exists for now)
    RobocopyLogsTui(RobocopyLogsTuiArgs),
}

impl Command {
    /// Invoke the command with global arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if tracing initialization fails or the command execution fails.
    pub fn invoke(self, global_args: &GlobalArgs) -> eyre::Result<()> {
        let json_behaviour = global_args.json_log_behaviour();
        // Call the logging helper from the logging module to initialize tracing.
        crate::logging::init_tracing(global_args.log_level(), &json_behaviour)?;
        match self {
            Command::RobocopyLogsTui(args) => args.invoke(),
        }
    }
}

impl ToArgs for Command {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            Command::RobocopyLogsTui(logs_args) => {
                args.push("robocopy-logs-tui".into());
                args.extend(logs_args.to_args());
            }
        }
        args
    }
}

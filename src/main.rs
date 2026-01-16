//! Simple CLI entrypoint that mirrors `teamy-mft` main behavior.
//! - Installs color-eyre for better error reports
//! - Parses CLI via `clap` using the `Cli` type from `crate::cli`
//! - Optionally attaches to an existing console when `--console-pid` is used
//! - Invokes the selected command
use clap::CommandFactory;
use clap::FromArgMatches;
use eyre::Result;
use teamy_robocopy::cli::Cli;
use teamy_windows::console::console_attach;

/// Entrypoint for the program to reduce coupling to the name of this crate.
///
/// # Errors
///
/// Returns an error if CLI parsing or command execution fails.
fn main() -> Result<()> {
    // Install error/reporting hooks
    color_eyre::install()?;

    // Build clap command and parse args into our `Cli` type
    let clap_cmd = Cli::command();
    let cli = Cli::from_arg_matches(&clap_cmd.get_matches())?;

    // If requested, attach to an existing console (hidden global flag)
    if let Some(pid) = cli.global_args.console_pid {
        console_attach(pid)?;
    }

    // Invoke the requested command
    cli.invoke()?;

    Ok(())
}

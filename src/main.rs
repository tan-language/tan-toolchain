mod format;
mod lint;
mod repl;
mod run;

use clap::{Arg, Command};
use format::handle_format;
use lint::handle_lint;
use tracing_subscriber::util::SubscriberInitExt;

use crate::repl::handle_repl;
use crate::run::handle_run;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .without_time()
        .with_target(false)
        .finish()
        .init();
}

fn main() -> anyhow::Result<()> {
    init_tracing();

    let run_cmd = Command::new("run").about("Run a Tan program").arg(
        Arg::new("PATH")
            .help("The path of the program")
            .required(true)
            .index(1),
    );

    let lint_cmd = Command::new("lint").about("Lint a Tan text file").arg(
        Arg::new("PATH")
            .help("The path of the text")
            .required(true)
            .index(1),
    );

    let format_cmd = Command::new("format").about("Format a Tan text file").arg(
        Arg::new("PATH")
            .help("The path of the text")
            .required(true)
            .index(1),
    );

    let tan_cmd = Command::new("tan")
        .bin_name("tan")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for the Tan Language")
        .subcommand(run_cmd)
        .subcommand(lint_cmd)
        .subcommand(format_cmd);

    let matches = tan_cmd.get_matches();

    if let Some(run_matches) = matches.subcommand_matches("run") {
        handle_run(run_matches)?;
    } else if let Some(lint_matches) = matches.subcommand_matches("lint") {
        handle_lint(lint_matches)?;
    } else if let Some(format_matches) = matches.subcommand_matches("format") {
        handle_format(format_matches)?;
    } else {
        handle_repl()?;
    }

    Ok(())
}

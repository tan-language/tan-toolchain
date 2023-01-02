mod lint;
mod repl;
mod run;
mod util;

use clap::{Arg, Command};
use lint::handle_lint;

use crate::repl::handle_repl;
use crate::run::handle_run;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()> {
    // #TODO consider a different name? even though 'run' is generic enough.
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

    let tan_cmd = Command::new("tan")
        .bin_name("tan")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for the Tan Language")
        .subcommand(run_cmd)
        .subcommand(lint_cmd);

    let matches = tan_cmd.get_matches();

    if let Some(run_matches) = matches.subcommand_matches("run") {
        // #TODO also handle run if a single `.tan` filename is passed.
        handle_run(run_matches)?;
    } else if let Some(lint_matches) = matches.subcommand_matches("lint") {
        handle_lint(lint_matches)?;
    } else {
        handle_repl()?;
    }

    Ok(())
}

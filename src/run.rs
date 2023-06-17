use std::path::Path;

use clap::ArgMatches;
use tan::eval::{env::Env, util::eval_module};
use tan_fmt::format_error_pretty;

// #TODO try to reuse the code from "use".
/// Read and evaluate a Tan program file.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let mut env = Env::prelude();
    let path = Path::new(path);
    let result = eval_module(path, &mut env);
    if let Err(errors) = result {
        let mut error_strings = Vec::new();

        for error in errors {
            // #TODO temp solution, can we optimize?
            let input = std::fs::read_to_string(&error.file_path)?;
            error_strings.push(format!("ERROR: {}", format_error_pretty(&error, &input)));
        }

        // #TODO use tracing::info!()
        eprintln!("{}", error_strings.join("\n\n"));
    };

    Ok(())
}

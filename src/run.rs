use std::path::Path;

use clap::ArgMatches;
use tan::{context::Context, error::ErrorKind, eval::util::eval_module};
use tan_formatting::{format_error, format_error_pretty};

/// Read and evaluate a Tan program file.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches.get_one("PATH").unwrap();

    let path = Path::new(path);

    // #todo handle general URLs, not only file://

    let path = std::fs::canonicalize(path)?;
    let path = format!("file://{}", path.to_string_lossy());

    let mut context = Context::new();

    let result = eval_module(path, &mut context);

    if let Err(errors) = result {
        let mut error_strings = Vec::new();

        for error in errors {
            match error.kind() {
                ErrorKind::FailedUse(_module_path, inner_errors) => {
                    let mut strings = Vec::new();

                    strings.push(format!("ERROR: {}", format_error(&error)));
                    for inner_error in inner_errors {
                        strings.push(format!("       + {}", format_error(&inner_error)));
                    }

                    error_strings.push(strings.join(""));
                }
                _ => {
                    // #TODO temp solution, can we optimize?
                    if let Ok(input) = std::fs::read_to_string(&error.file_path) {
                        error_strings
                            .push(format!("ERROR: {}", format_error_pretty(&error, &input)));
                    } else {
                        error_strings.push(format!(
                            "ERROR: {} note: Cannot read the source file",
                            format_error(&error)
                        ));
                    }
                }
            }
        }

        // #TODO use tracing::info!()
        eprintln!("{}", error_strings.join("\n\n"));
    };

    Ok(())
}

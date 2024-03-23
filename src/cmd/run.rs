use std::{path::Path, rc::Rc};

use clap::ArgMatches;
use tan::{
    context::Context,
    error::{Error, ErrorVariant},
    eval::util::eval_module,
    expr::Expr,
};

use crate::util::{format_error_pretty, format_error_short, format_panic_pretty};

// #todo use another name.
// #todo temp solution, can we optimize?
fn format_error(error: &Error) -> String {
    if let Ok(input) = std::fs::read_to_string(&error.file_path) {
        format!("ERROR: {}", format_error_pretty(error, &input))
    } else {
        format!("ERROR: {}", format_error_short(error))
    }
}

/// Read and evaluate a Tan program file.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches.get_one("PATH").unwrap();

    // Extracts arguments following the `--` separator. These arguments are passed
    // as the `**process-args**` to the program.

    let program_args: Vec<&String> =
        if let Some(program_args) = run_matches.get_many("program_args") {
            program_args.collect()
        } else {
            Vec::new()
        };

    let path = Path::new(path);

    // #todo handle general URLs, not only file://

    let path = std::fs::canonicalize(path)?;
    let path = format!("file://{}", path.to_string_lossy());

    let mut context = Context::new();

    // #todo #hack this is a temp solution.
    // #todo consider capital letters for 'magic'/external constants.
    let process_args: Vec<Expr> = program_args.into_iter().map(Expr::string).collect();
    context
        .top_scope
        .insert("**process-args**", Rc::new(Expr::array(process_args)));

    let result = eval_module(path, &mut context, false);

    // #todo show better error if file not found.

    if let Err(errors) = result {
        let mut error_strings = Vec::new();

        for error in errors {
            match error.variant() {
                ErrorVariant::FailedUse(_module_path, inner_errors) => {
                    error_strings.push(format_error(&error));
                    for inner_error in inner_errors {
                        error_strings.push(format_error(inner_error));
                    }
                }
                ErrorVariant::Panic(..) => {
                    error_strings.push(format!("PANIC: {}", format_panic_pretty(&error)));
                }
                _ => {
                    error_strings.push(format_error(&error));
                }
            }
        }

        // #todo use tracing::error!()
        eprintln!("{}", error_strings.join("\n\n"));
    };

    Ok(())
}

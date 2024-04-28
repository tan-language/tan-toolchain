use std::{path::Path, sync::Arc};

use clap::ArgMatches;
use tan::{context::Context, eval::util::eval_module, expr::Expr};

use crate::util::report::report_errors;

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

    // #todo setup CURRENT_MODULE_PATH, CURRENT_FILE_PATH?
    // #todo setup PROFILE

    // #todo #hack this is a temp solution.
    // #todo consider capital letters for 'magic'/external constants.
    // #todo avoid ** for non dynamically-scoped variables.
    let process_args: Vec<Expr> = program_args.into_iter().map(Expr::string).collect();
    context
        .top_scope
        .insert("**process-args**", Arc::new(Expr::array(process_args)));

    let result = eval_module(path, &mut context, false);

    // #todo show better error if file not found.

    if let Err(errors) = result {
        report_errors(&errors, None);
    };

    Ok(())
}

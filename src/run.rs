use std::{fs, path::Path};

use clap::ArgMatches;
use tan::{
    ann::Ann,
    api::{has_tan_extension, resolve_string},
    eval::{env::Env, eval},
    expr::Expr,
};
use tracing::error;

use crate::util::eval_string_with_error_report;

fn eval_file(path: &str) {
    let input = std::fs::read_to_string(path).expect("cannot read input");

    let mut env = Env::prelude();

    eval_string_with_error_report(&input, &mut env);
}

// #TODO try to reuse the code from "use".
/// Read and evaluate a Tan program file.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let path2 = Path::new(path);

    // #TODO also try to automatically add the .tan or emoji extension.

    if !path2.exists() {
        error!("Path `{path}` does not exist.");
    } else if has_tan_extension(path) {
        eval_file(path);
    } else if path2.is_dir() {
        // #TODO report error if it's not a directory but a file with unsupported extension.
        // #TODO not working correctly yet, need to passes, first definitions, then eval.
        let file_paths = fs::read_dir(path)?;

        let mut resolved_exprs: Vec<Ann<Expr>> = Vec::new();

        let mut env = Env::prelude();

        for file_path in file_paths {
            let path = file_path?.path();

            if !path.display().to_string().ends_with(".tan") {
                continue;
            }

            // #TODO handle the range of the error.
            let input = std::fs::read_to_string(path)?;

            let result = resolve_string(input, &mut env);

            let Ok(mut exprs) = result else {
                let err = result.unwrap_err();
                // #TODO better error handling here!
                dbg!(&err);
                // #TODO maybe continue parsing/resolving to find more errors?
                // #TODO better error here!
                return Err(tan::error::Error::FailedUse.into());
            };

            resolved_exprs.append(&mut exprs);
        }

        for expr in resolved_exprs {
            if let Err(err) = eval(&expr, &mut env) {
                // #TODO better error handling here!
                dbg!(&err);
                // #TODO better error here!
                return Err(tan::error::Error::FailedUse.into());
            }
        }
    } else {
        error!("Path `{path}` is not a valid module, unrecognized extension.");
    }

    // let result = eval_module(path);
    //
    // if let Err(errors) = result {
    //     // eprintln!("{}", format_errors(&input, &errors));
    //     dbg!(errors);
    // };

    Ok(())
}

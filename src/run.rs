use std::fs;

use clap::ArgMatches;
use once_cell::sync::Lazy;
use regex::Regex;
use tan::{
    ann::Ann,
    api::resolve_string,
    eval::{env::Env, eval},
    expr::Expr,
};

use crate::util::eval_string_with_error_report;

// #Insight
// No need to handle shebang in the reader (lexer, parser).

pub static SHEBANG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^#!(.*)\n").unwrap());

// #TODO skip_shebang messes with the lexer ranges, FIX!
/// Skip the 'shebang' line, if it exists.
fn skip_shebang(input: String) -> String {
    if input.starts_with("#!") {
        SHEBANG_RE.replace(&input, "").to_string()
    } else {
        input
    }
}

fn eval_file(path: &str) {
    let input = std::fs::read_to_string(path).expect("cannot read input");

    let input = skip_shebang(input);

    let mut env = Env::prelude();

    eval_string_with_error_report(&input, &mut env);
}

// #TODO try to reuse the code from "use".
/// Read and evaluate a Tan program file.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches
        .get_one("PATH")
        .expect("missing path to program file");

    if path.ends_with(".tan") {
        eval_file(path);
    } else {
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
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::skip_shebang;

    #[test]
    fn skip_shebang_ignores_a_shebang_line() {
        let input = "#!tan\n(let a 1)\n a".to_string();
        assert_eq!(skip_shebang(input), "(let a 1)\n a")
    }

    #[test]
    fn skip_shebang_handles_no_shebang_line() {
        let input = "(let a 1)\n a".to_string();
        assert_eq!(skip_shebang(input), "(let a 1)\n a")
    }
}

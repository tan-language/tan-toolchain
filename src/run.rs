use std::fs;

use clap::ArgMatches;
use once_cell::sync::Lazy;
use regex::Regex;
use tan::eval::env::Env;

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

        for file_path in file_paths {
            let path = file_path?.path().display().to_string();

            if !path.ends_with(".tan") {
                continue;
            }

            eval_file(&path);
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

use clap::ArgMatches;
use regex::Regex;
use tan::eval::{env::Env, prelude::setup_prelude};

use crate::util::eval_string;

/// Skip the 'shebang' line, if it exists.
fn skip_shebang(input: String) -> String {
    if input.starts_with("#!") {
        // #TODO 'cache' with once_cell
        let shebang_re = Regex::new("^#!(.*)\n").unwrap();
        shebang_re.replace(&input, "").to_string()
    } else {
        input
    }
}

// #TODO properly implement this.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path).expect("cannot read input");

    let input = skip_shebang(input);

    let mut env = setup_prelude(Env::default());

    eval_string(&input, &mut env);

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

use clap::ArgMatches;
use tan::api::parse_string_all;

pub fn handle_lint(lint_matches: &ArgMatches) -> anyhow::Result<()> {
    // #TODO extract and reuse handling of file or dir from other commands.

    let path: &String = lint_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path)?;
    let _result = parse_string_all(&input);

    // let diagnostics = match result {
    //     Ok(exprs) => {
    //         let mut diagnostics = Vec::new();

    //         let mut lint = SnakeCaseNamesLint::new(&input);
    //         lint.run(&exprs);
    //         diagnostics.append(&mut lint.diagnostics);

    //         diagnostics
    //     }
    //     Err(errors) => compute_parse_error_diagnostics(&input, errors)?,
    // };

    // Ok(diagnostics)

    Ok(())
}

use clap::ArgMatches;
use tan::api::parse_string_all;
use tan_lints::{compute_diagnostics, Diagnostic, DiagnosticSeverity};

fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    // #todo improve the formatting.
    format!(
        "{:?} [line: {}, col: {}]: {}",
        diagnostic.severity.unwrap_or(DiagnosticSeverity::WARNING),
        diagnostic.range.start.line,
        diagnostic.range.start.character,
        diagnostic.message
    )
}

pub fn handle_lint(lint_matches: &ArgMatches) -> anyhow::Result<()> {
    // #todo extract and reuse handling of file or dir from other commands.

    let path: &String = lint_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path)?;
    let parse_result = parse_string_all(input);
    let diagnostics = compute_diagnostics(&parse_result);

    for diagnostic in diagnostics {
        println!("{}", format_diagnostic(&diagnostic));
    }

    Ok(())
}

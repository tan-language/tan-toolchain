use clap::ArgMatches;
use tan::api::parse_string_all;
use tan_formatting::pretty::Formatter;

pub fn handle_format(lint_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = lint_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path)?;

    let Ok(exprs) = parse_string_all(input) else {
        return Err(anyhow::anyhow!("cannot parse the file"));
    };

    let formatter = Formatter::new(&exprs);
    let formatted = formatter.format();

    print!("{formatted}");

    Ok(())
}

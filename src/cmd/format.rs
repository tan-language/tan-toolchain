use clap::ArgMatches;
use tan_analysis::parsing::parse_string_for_analysis;
use tan_formatting::pretty::Formatter;

pub fn handle_format(format_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = format_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path)?;

    let Ok(exprs) = parse_string_for_analysis(input) else {
        return Err(anyhow::anyhow!("cannot parse the file"));
    };

    let formatter = Formatter::new(&exprs);
    let formatted = formatter.format();

    print!("{formatted}");

    Ok(())
}

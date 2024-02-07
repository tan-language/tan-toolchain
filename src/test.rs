use clap::ArgMatches;

pub fn handle_test(_test_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("-- TEST --");
    // let path: &String = lint_matches
    //     .get_one("PATH")
    //     .expect("missing path to program file");

    // let input = std::fs::read_to_string(path)?;

    // let Ok(exprs) = parse_string_all(input) else {
    //     return Err(anyhow::anyhow!("cannot parse the file"));
    // };

    // let formatter = Formatter::new(&exprs);
    // let formatted = formatter.format();

    // print!("{formatted}");

    Ok(())
}

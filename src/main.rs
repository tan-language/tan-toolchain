use clap::{Arg, Command};
use tan::{lexer::Lexer, parser::Parser};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn repl() {}

fn main() -> anyhow::Result<()> {
    // #TODO consider a different name? even though 'run' is generic enough.
    let run_cmd = Command::new("run").about("Run a Tan program").arg(
        Arg::new("PATH")
            .help("The path of the program")
            .required(true)
            .index(1),
    );

    let tan_cmd = Command::new("tan")
        .bin_name("tan")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for Tan")
        .subcommand(run_cmd);

    let matches = tan_cmd.get_matches();

    if let Some(run_matches) = matches.subcommand_matches("run") {
        let path: &String = run_matches
            .get_one("PATH")
            .expect("missing path to program file");

        let input = std::fs::read_to_string(path).expect("cannot read input");

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.lex().expect("cannot lex");
        dbg!(&tokens);

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        dbg!(&expr);
    } else {
        // #TODO repl!
    }

    Ok(())
}

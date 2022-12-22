use clap::{Arg, ArgMatches, Command};
use rustyline::{error::ReadlineError, Editor};
use tan::{lexer::Lexer, parser::Parser};
use tan_fmt::compact::format_compact;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// #TODO properly implement this.
fn run(run_matches: &ArgMatches) -> anyhow::Result<()> {
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

    Ok(())
}

// #TODO rename to `shell` or something else?
fn repl() -> anyhow::Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        // #TODO what would be a cool prompt?
        // #TODO try to use the legendary `READY` in some capacity.
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut lexer = Lexer::new(&line);
                let tokens = lexer.lex();

                let Ok(tokens) = tokens else {
                    println!("Parse error: {}", tokens.unwrap_err());
                    continue;
                };

                let mut parser = Parser::new(tokens);
                let expr = parser.parse();

                let Ok(expr) = expr else {
                    println!("Parse error: {}", expr.unwrap_err());
                    continue;
                };

                println!("{}", format_compact(expr.as_ref()));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").unwrap();

    Ok(())
}

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
        run(run_matches)?;
    } else {
        repl()?;
    }

    Ok(())
}

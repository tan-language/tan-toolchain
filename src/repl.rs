use std::io::{stdout, Write};

use rustyline::{error::ReadlineError, DefaultEditor};
use tan::{api::eval_string, eval::env::Env, expr::Expr};
use tan_fmt::format_error_pretty;

const HISTORY_FILENAME: &str = ".tan_history.txt";

// #TODO if we use an array for %i and %o, we can apply all the Seq functions, potentially useful! but it's less ergonomic.
// #TODO don't advance the line-variable if the input is just a comment
// #TODO `---` comments are convenient as separators.

// #TODO rename to `shell` or something else?
pub fn handle_repl() -> anyhow::Result<()> {
    // #TODO support completer!

    // let config = Config::builder()
    //     .history_ignore_space(true)
    //     .completion_type(CompletionType::List)
    //     .edit_mode(EditMode::Emacs)
    //     .build();
    // let h = MyHelper {
    //     completer: FilenameCompleter::new(),
    //     highlighter: MatchingBracketHighlighter::new(),
    //     hinter: HistoryHinter {},
    //     colored_prompt: "".to_owned(),
    //     validator: MatchingBracketValidator::new(),
    // };
    // let mut rl = Editor::with_config(config)?;
    // rl.set_helper(Some(h));
    // rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    // rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }

    let mut rl = DefaultEditor::new()?;

    if rl.load_history(HISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }

    println!("Tan, press CTRL-D to exit.");

    let mut env = Env::prelude();

    let mut index = 0;

    loop {
        // #TODO what would be a cool prompt? (Wolfram Language has an interesting prompt).
        // #TODO have prefix for output/result also.
        // #TODO try to use the legendary `READY` in some capacity.
        let readline = rl.readline(&format!("{index}> "));

        match readline {
            Ok(input) => {
                rl.add_history_entry(&input)?;

                // #TODO find better input variable name.
                // #TODO use input list/array, like wolfram, e.g. (*in* 1), nah too difficult to type!
                env.insert(format!("$i{index}"), Expr::String(input.clone()));

                let result = eval_string(&input, &mut env);

                let Ok(value) = result else {
                    let errors = result.unwrap_err();

                    let mut error_strings = Vec::new();
                    for error in errors {
                        error_strings.push(format!("ERROR: {}", format_error_pretty(&error, &input)));
                    }

                    eprintln!("{}", error_strings.join("\n\n"));

                    continue;
                };

                // #TODO find better output variable name.
                // #TODO use output list/array, like wolfram, e.g. (*out* 1)
                env.insert(format!("$o{index}"), value.clone());

                match value {
                    Expr::One => (),
                    _ => println!("{value}"),
                }

                let _ = stdout().flush();
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
                eprintln!("ERROR: {err:?}");
                break;
            }
        }

        index += 1;
    }

    // #TODO could we trap the (exit)?
    rl.save_history(HISTORY_FILENAME).unwrap();

    Ok(())
}

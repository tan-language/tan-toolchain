use std::io::{stdout, Write};

use crate::util::format_error_pretty;
use rustyline::{error::ReadlineError, DefaultEditor};
use tan::{api::eval_string, context::Context, expr::Expr};

const HISTORY_FILENAME: &str = ".tan-history.txt";

// #todo handle panic errors.
// #todo if we use an array for %i and %o, we can apply all the Seq functions, potentially useful! but it's less ergonomic.
// #todo don't advance the line-variable if the input is just a comment
// #todo `---` comments are convenient as separators.

// #todo rename to `shell` or something else?
pub fn handle_repl() -> anyhow::Result<()> {
    // #todo support completer!

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

    let mut context = Context::new();

    let mut index = 0;

    loop {
        // #todo what would be a cool prompt? (Wolfram Language has an interesting prompt).
        // #todo have prefix for output/result also.
        // #todo try to use the legendary `READY` in some capacity.
        let readline = rl.readline(&format!("{index}> "));

        match readline {
            Ok(input) => {
                rl.add_history_entry(&input)?;

                // #todo find better input variable name.
                // #todo use input list/array, like wolfram, e.g. (*in* 1), nah too difficult to type!
                context
                    .scope
                    .insert(format!("$i{index}"), Expr::String(input.clone()));

                // #insight this version of eval_string does not create a new module for each input, which is what we want.
                let result = eval_string(&input, &mut context);

                let Ok(value) = result else {
                    let errors = result.unwrap_err();

                    let mut error_strings = Vec::new();
                    for error in errors {
                        error_strings
                            .push(format!("ERROR: {}", format_error_pretty(&error, &input)));
                    }

                    eprintln!("{}", error_strings.join("\n\n"));

                    continue;
                };

                // #todo find better output variable name.
                // #todo use output list/array, like wolfram, e.g. (*out* 1)
                context.scope.insert(format!("$o{index}"), value.clone());

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
                eprintln!("{err:?}");
                break;
            }
        }

        index += 1;
    }

    // #todo could we trap the (exit)?
    rl.save_history(HISTORY_FILENAME).unwrap();

    Ok(())
}

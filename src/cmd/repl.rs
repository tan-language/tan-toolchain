use std::io::{stdout, Write};

use rustyline::{error::ReadlineError, DefaultEditor};
use tan::{
    api::compile_string,
    context::Context,
    eval::eval,
    expr::Expr,
    util::standard_names::{CURRENT_FILE_PATH, CURRENT_MODULE_PATH},
};

use crate::util::{canonicalize::canonicalize_input, report::report_errors};

// #todo consider a different extensions, e.g. *.text
// #todo consider saving history in tan (sexp) format.
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

    // Initialize some global variables.

    // #todo refactor this initialization.

    // #todo what dummy value to use instead of `REPL`?
    context
        .top_scope
        .insert(CURRENT_FILE_PATH, Expr::string("REPL"));

    let current_dir = std::env::current_dir()?.display().to_string();
    context
        .top_scope
        .insert(CURRENT_MODULE_PATH, Expr::string(current_dir));

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

                let result = compile_string(&input, &mut context);

                let Ok(exprs) = result else {
                    let errors = result.unwrap_err();
                    report_errors(&errors, Some(&input));
                    continue;
                };

                // #todo pass the context also?
                let expr = canonicalize_input(exprs);

                // let result = eval_string(&input, &mut context);
                let result = eval(&expr, &mut context);

                // #todo handle panic.
                // #todo use the same code as run.

                let Ok(value) = result else {
                    let error = result.unwrap_err();
                    report_errors(&[error], Some(&input));
                    continue;
                };

                // #todo find better output variable name.
                // #todo use output list/array, like wolfram, e.g. (*out* 1)
                context.scope.insert(format!("$o{index}"), value.clone());

                match value {
                    Expr::None => (),
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

// #todo find a better filename.

use tan::error::{Error, ErrorVariant};

use crate::util::{
    ansi::{bold, red},
    format_error_pretty, format_error_short, format_panic_pretty,
};

// #todo find a better name.
// #todo temp solution, can we optimize?
fn format_error_string(error: &Error) -> String {
    let error_str = if let Ok(input) = std::fs::read_to_string(&error.file_path) {
        format_error_pretty(error, &input)
    } else {
        format_error_short(error)
    };
    format!("{} {}", bold(red("error:")), error_str)
}

// #todo reuse from_error_string.
// #todo find a better name.
// #todo temp solution, can we optimize?
fn format_panic_string(error: &Error) -> String {
    // let error_str = if let Ok(input) = std::fs::read_to_string(&error.file_path) {
    //     format_error_pretty(error, &input)
    // } else {
    //     format_error_short(error)
    // };
    format!("{} {}", bold(red("panic:")), format_panic_pretty(error))
}

// #todo add unit tests.
pub fn report_errors(errors: &[Error]) {
    let mut error_strings = Vec::new();

    for error in errors {
        match error.variant() {
            ErrorVariant::FailedUse(_module_path, inner_errors) => {
                error_strings.push(format_error_string(error));
                for inner_error in inner_errors {
                    error_strings.push(format_error_string(inner_error));
                }
            }
            ErrorVariant::Panic(..) => {
                error_strings.push(format_panic_string(error));
                eprintln!("{}", error_strings.join("\n\n"));
                // #insight
                // - https://users.rust-lang.org/t/solved-why-101-exit-code-when-use-panic/80061
                // - https://tldp.org/LDP/abs/html/exitcodes.html
                // #todo think about the exit code!
                std::process::exit(101);
            }
            _ => {
                error_strings.push(format_error_string(error));
            }
        }
    }

    // #todo use tracing::error!()
    eprintln!("{}", error_strings.join("\n\n"));
}

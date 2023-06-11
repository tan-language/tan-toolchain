use tan::{ann::Ann, api::eval_string, error::Error, eval::env::Env, expr::Expr, range::Ranged};
use tan_fmt::format_error_pretty;

#[allow(dead_code)]
pub fn eval_string_with_error_report(input: &str, env: &mut Env) -> Option<Expr> {
    let result = eval_string(input, env);

    match result {
        Ok(Ann(expr, ..)) => Some(expr),
        Err(errors) => {
            // #TODO extract this as a method.
            let mut error_strings = Vec::new();
            for error in errors {
                error_strings.push(format!(
                    "ERROR: {}",
                    format_error_pretty(&error, input, None)
                ));
            }
            eprintln!("{}", error_strings.join("\n\n"));
            None
        }
    }
}

#[allow(dead_code)]
pub fn format_errors(errors: &[Ranged<Error>]) -> String {
    let mut error_strings = Vec::new();

    // #TODO how to format the errors here?
    for error in errors {
        error_strings.push(format!(
            "ERROR: {}",
            error,
            // format_error_pretty(&error, input, None)
        ));
    }

    format!("{}", error_strings.join("\n\n"))
}

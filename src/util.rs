use tan::{ann::Ann, api::eval_string, error::Error, eval::env::Env, expr::Expr};
use tan_fmt::format_error_pretty;

#[allow(dead_code)]
pub fn eval_string_with_error_report(input: &str, env: &mut Env) -> Option<Expr> {
    let result = eval_string(input, env);

    match result {
        Ok(Ann(expr, ..)) => Some(expr),
        Err(errors) => {
            eprintln!("{}", format_errors(&errors));
            None
        }
    }
}

#[allow(dead_code)]
pub fn format_errors(errors: &[Error]) -> String {
    let mut error_strings = Vec::new();

    for error in errors {
        error_strings.push(format!("ERROR: {}", format_error_pretty(&error)));
    }

    format!("{}", error_strings.join("\n\n"))
}

use tan::{ann::Ann, api::eval_string, eval::env::Env, expr::Expr};
use tan_fmt::format_error_pretty;

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

// pub fn format_errors(input: &str, errors: &[Ranged<Error>]) -> String {
//     let mut error_strings = Vec::new();

//     for error in errors {
//         error_strings.push(format!(
//             "ERROR: {}",
//             format_error_pretty(&error, input, None)
//         ));
//     }

//     format!("{}", error_strings.join("\n\n"))
// }

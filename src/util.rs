use tan::{api::eval_string, eval::env::Env, expr::Expr};
use tan_fmt::format_error_pretty;

pub fn eval_string_with_error_report(input: &str, env: &mut Env) -> Option<Expr> {
    let result = eval_string(input, env);

    match result {
        Ok(expr) => Some(expr),
        Err(err) => {
            eprintln!("{}", format_error_pretty(&err, input, None));
            None
        }
    }
}

use tan::{api::eval_string, error::Error, eval::env::Env, expr::Expr};
use tan_fmt::format_error_pretty;

pub fn eval_string_with_error_report(input: &str, env: &mut Env) -> Option<Expr> {
    let result = eval_string(input, env);

    match result {
        Ok(expr) => Some(expr),
        Err(Error::Lexical(err)) => {
            eprintln!("{}", format_error_pretty(&err, input, None));
            None
        }
        Err(Error::Parse(err)) => {
            eprintln!("{}", format_error_pretty(&err, input, None));
            None
        }
        Err(Error::Eval(err)) => {
            eprintln!("{err}");
            None
        }
    }
}

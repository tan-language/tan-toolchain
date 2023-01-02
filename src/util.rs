use tan::{
    eval::{env::Env, eval},
    expr::Expr,
    lexer::Lexer,
    parser::Parser,
};
use tan_fmt::format_error_pretty;

pub fn eval_string(input: &str, env: &mut Env) -> Option<Expr> {
    let mut lexer = Lexer::new(input);
    let result = lexer.lex();

    let Ok(tokens) = result else {
        eprintln!("{}", format_error_pretty(&result.unwrap_err(), input, None));
        return None;
    };

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    let Ok(expr) = result else {
        eprintln!("{}", format_error_pretty(&result.unwrap_err(), input, None));
        return None;
    };

    let result = eval(expr, env);

    let Ok(value) = result else {
        eprintln!("error: {}", result.unwrap_err());
        return None;
    };

    Some(value)
}

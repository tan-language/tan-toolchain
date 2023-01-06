use tan::{
    eval::{env::Env, eval},
    expr::Expr,
    lexer::Lexer,
    parser::Parser,
};
use tan_fmt::format_error_pretty;

// #TODO move to `tan` crate
// #TODO handle errors externally to this function
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
        // #TODO use format_error_pretty!
        eprintln!("{}", result.unwrap_err());
        return None;
    };

    Some(value)
}

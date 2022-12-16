use tan::{lexer::Lexer, parser::Parser};

fn main() {
    let input = std::fs::read_to_string("input.tan").expect("cannot read input");

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.lex().expect("cannot lex");
    dbg!(&tokens);

    let mut parser = Parser::new(&tokens);
    let expr = parser.parse();
    dbg!(&expr);
}

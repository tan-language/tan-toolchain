use tan::lexer::Lexer;

fn main() {
    let input = std::fs::read_to_string("input.tan").expect("cannot read input");
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.lex().expect("cannot lex");
    dbg!(tokens);
}

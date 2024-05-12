// #unused #warning not used at the moment, I don't think we can get this to work.

// #idea use special character to trigger canonicalization?
// #insight canonicalization can be useful in tan-shell.

// #insight canonicalization happend after parsing.

// #todo find another name than canonicalization.

// #insight
// We expect one expression, in contexts like REPL or the Shell. If the input
// consists of more expressions, try to auto-wrap with parens.

// #todo consider extracting as a helper, use in more places.
// #todo find a more descriptive name.
// #todo return the canonicalized input string also
fn canonicalize_input(input: Vec<Expr>) -> (Expr, String) {
    if input.len() > 1 {
        todo!()
    }
}

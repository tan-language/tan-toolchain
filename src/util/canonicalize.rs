// #insight canonicalization can be useful in tan-shell.

// #insight canonicalization needs to happen after analysis.

// #todo find another name than canonicalization, how about `expand`, or `collapse` (to a single expression)?
// #todo maybe canonicaliza is not that bad after all.

// #insight
// We expect one expression, in contexts like REPL or the Shell. If the input
// consists of more expressions, try to auto-wrap with parens or convert to
// a single (do ...) expression.

use tan::expr::Expr;

// #insight no need to update the input string, the token ranges are stable!
// #todo consider extracting as a helper, use in more places.
// #todo find a more descriptive name.
// #todo add unit-tests
pub fn canonicalize_exprs(exprs: Vec<Expr>) -> Expr {
    match exprs.len() {
        0 => Expr::None,
        1 => {
            let mut exprs = exprs;
            exprs.pop().unwrap()
        }
        _ => {
            // #insight
            // Two opportunities for canonicalization:
            // - missing parens
            // - multiple expressions
            // #todo check if the symbol is defined!
            if exprs[0].as_symbolic().is_some() {
                // #insight no need to update the input string, ranges etc stay the same.
                Expr::List(exprs)
            } else {
                // convert multiple expressions into a single (do ...) expression.
                // #insight no need to update the input string, ranges etc stay the same.
                let mut exprs = exprs;
                exprs.insert(0, Expr::symbol("do"));
                Expr::List(exprs)
            }
        }
    }
}

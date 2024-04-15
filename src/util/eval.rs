// use tan::{api::eval_string, context::Context, error::Error, expr::Expr};

// // #todo move to tan::api?
// pub fn eval_file(path: impl AsRef<str>) -> Result<Expr, Vec<Error>> {
//     // #todo use eval_module here!!
//     // #todo this unwrap should probably get removed.
//     let input = std::fs::read_to_string(path.as_ref()).unwrap();
//     let mut context = Context::new();
//     eval_string(input, &mut context)
// }

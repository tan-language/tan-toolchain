use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use clap::ArgMatches;
use tan::{
    context::Context,
    eval::{invoke_func, util::eval_module},
    expr::{format_value, Expr},
    util::standard_names::CURRENT_FILE_PATH,
};

use crate::util::{
    ansi::{bold, green, red},
    report::report_errors,
};

// cargo r -- test tests/fixtures/test-fixture

// #todo add unit tests to verify error logging, etc.
// #todo use a different name than test: spec, conformance, something else?
// #todo *.test.tan files should be ignored on non-test-profile runs.
// #todo attach description annotations to unit-test functions to be printed when the tests run, is this really useful?

// #todo reuse run cmd infrastructure.

pub fn handle_test(test_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = test_matches.get_one("PATH").unwrap();

    let path = Path::new(path);

    // #todo handle general URLs, not only file://

    let path = std::fs::canonicalize(path)?;
    let path = format!("file://{}", path.to_string_lossy());

    let mut context = Context::new();

    let test_failures: Arc<RwLock<Vec<Expr>>> = Arc::new(RwLock::new(Vec::new()));

    context
        .dynamic_scope
        .insert("*test-failures*", Expr::Array(test_failures.clone()));

    // #todo setup CURRENT_MODULE_PATH, CURRENT_FILE_PATH?
    // #todo setup PROFILE

    let result = eval_module(path, &mut context, false);

    if let Err(errors) = result {
        report_errors(&errors, None);
        // #todo flag the error here
        return Ok(());
    };

    let expr = result.unwrap();
    let Expr::Module(module) = expr.unpack() else {
        // #todo proper error-handling.
        panic!("error");
    };

    for (name, value) in module.scope.bindings.read().unwrap().iter() {
        // #insight #temp test-case methods start with test.
        if name.starts_with("test") {
            if let Expr::Func(_, _, _, file_path) = value.unpack() {
                test_failures.write().unwrap().clear();
                print!("test `{name}` in `{file_path}` ");

                let old_current_file_path = context.top_scope.get(CURRENT_FILE_PATH);
                context
                    .top_scope
                    .insert(CURRENT_FILE_PATH, Expr::string(file_path));

                // #todo will need to pass arguments by ref.
                let result = invoke_func(value, Vec::new(), &mut context);

                if let Err(error) = result {
                    report_errors(&[error], None);
                    // #todo flag the error here
                    return Ok(());
                };

                let failure_count = test_failures.write().unwrap().len();
                if failure_count > 0 {
                    println!("{}", bold(red("FAIL")));
                    for failure in test_failures.read().unwrap().iter() {
                        println!(
                            "{} {}",
                            bold(red("assertion failed:")),
                            format_value(failure)
                        );
                    }
                } else {
                    println!("{}", green("OK"));
                }

                if let Some(old_current_file_path) = old_current_file_path {
                    // #insight we should revert the previous current file, in case of 'use'
                    context
                        .top_scope
                        .insert(CURRENT_FILE_PATH, old_current_file_path.unpack().clone());
                }
            }
        }
    }

    Ok(())
}

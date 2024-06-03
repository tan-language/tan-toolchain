use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use clap::ArgMatches;
use tan::{
    context::Context,
    eval::{invoke_func, util::eval_module},
    expr::{format_value, Expr},
    util::standard_names::{CURRENT_FILE_PATH, PROFILE},
};

use crate::util::{
    ansi::{bold, green, red},
    fs::filter_walk_dir,
    report::report_errors,
};

// cargo r -- test tests/fixtures/test-fixture

// #todo recursively scan for test files!
// #todo add command-line option to disable recursive scan.

// #todo add unit tests to verify error logging, etc.
// #todo use a different name than test: spec, conformance, something else?
// #todo *.test.tan files should be ignored on non-test-profile runs.
// #todo attach description annotations to unit-test functions to be printed when the tests run, is this really useful?

// #todo reuse run cmd infrastructure.

// #todo find better name.
// #todo extract to tan-analysis
// #todo how to ensure tests are not run multiple times with module interdependencies, will need some work.
// #todo should return a tree? or at least have a version with a tree.
#[allow(dead_code)]
pub fn compute_module_paths(path: impl AsRef<Path>) -> Result<Vec<String>, std::io::Error> {
    let predicate = |p: &str| (!p.contains("/.git/")) && p.ends_with(".test.tan");
    filter_walk_dir(path.as_ref(), &predicate)
}

fn evaluate_test_module(path: &str) -> anyhow::Result<usize> {
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

    // #insight don't set CURRENT_MODULE_PATH, it will be set in eval_module.

    // let current_dir = std::env::current_dir()?.display().to_string();
    // context
    //     .top_scope
    //     .insert(CURRENT_MODULE_PATH, Expr::string(current_dir));

    // #todo use a constant/enum for the PROFILE value.
    // #todo add custom helper method to context to setup '!special!' values.
    context.top_scope.insert(PROFILE, Expr::string("test"));

    let result = eval_module(path, &mut context, false);

    if let Err(errors) = result {
        report_errors(&errors, None);
        // #todo flag the error here
        return Ok(0);
    };

    let expr = result.unwrap();
    let Expr::Module(module) = expr.unpack() else {
        // #todo proper error-handling.
        panic!("error");
    };

    let mut total_failure_count = 0;

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
                    return Ok(0);
                };

                let failure_count = test_failures.write().unwrap().len();
                if failure_count > 0 {
                    total_failure_count += failure_count;
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

    Ok(total_failure_count)
}

pub fn handle_test(test_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = test_matches.get_one("PATH").unwrap();

    // #todo recursively scan for directories/modules!
    // #todo extract as helper function, in analysis.

    // let paths = compute_module_paths(path);
    // dbg!(&paths.unwrap());

    // let path = Path::new(path);

    // // #todo handle general URLs, not only file://

    // let path = std::fs::canonicalize(path)?;
    // let path = format!("file://{}", path.to_string_lossy());

    // let mut context = Context::new();

    // let test_failures: Arc<RwLock<Vec<Expr>>> = Arc::new(RwLock::new(Vec::new()));

    // context
    //     .dynamic_scope
    //     .insert("*test-failures*", Expr::Array(test_failures.clone()));

    // // #todo setup CURRENT_MODULE_PATH, CURRENT_FILE_PATH?

    // // #insight don't set CURRENT_MODULE_PATH, it will be set in eval_module.

    // // let current_dir = std::env::current_dir()?.display().to_string();
    // // context
    // //     .top_scope
    // //     .insert(CURRENT_MODULE_PATH, Expr::string(current_dir));

    // // #todo use a constant/enum for the PROFILE value.
    // // #todo add custom helper method to context to setup '!special!' values.
    // context.top_scope.insert(PROFILE, Expr::string("test"));

    // let result = eval_module(path, &mut context, false);

    // if let Err(errors) = result {
    //     report_errors(&errors, None);
    //     // #todo flag the error here
    //     return Ok(());
    // };

    // let expr = result.unwrap();
    // let Expr::Module(module) = expr.unpack() else {
    //     // #todo proper error-handling.
    //     panic!("error");
    // };

    // let mut total_failure_count = 0;

    // for (name, value) in module.scope.bindings.read().unwrap().iter() {
    //     // #insight #temp test-case methods start with test.
    //     if name.starts_with("test") {
    //         if let Expr::Func(_, _, _, file_path) = value.unpack() {
    //             test_failures.write().unwrap().clear();
    //             print!("test `{name}` in `{file_path}` ");

    //             let old_current_file_path = context.top_scope.get(CURRENT_FILE_PATH);
    //             context
    //                 .top_scope
    //                 .insert(CURRENT_FILE_PATH, Expr::string(file_path));

    //             // #todo will need to pass arguments by ref.
    //             let result = invoke_func(value, Vec::new(), &mut context);

    //             if let Err(error) = result {
    //                 report_errors(&[error], None);
    //                 // #todo flag the error here
    //                 return Ok(());
    //             };

    //             let failure_count = test_failures.write().unwrap().len();
    //             if failure_count > 0 {
    //                 total_failure_count += failure_count;
    //                 println!("{}", bold(red("FAIL")));
    //                 for failure in test_failures.read().unwrap().iter() {
    //                     println!(
    //                         "{} {}",
    //                         bold(red("assertion failed:")),
    //                         format_value(failure)
    //                     );
    //                 }
    //             } else {
    //                 println!("{}", green("OK"));
    //             }

    //             if let Some(old_current_file_path) = old_current_file_path {
    //                 // #insight we should revert the previous current file, in case of 'use'
    //                 context
    //                     .top_scope
    //                     .insert(CURRENT_FILE_PATH, old_current_file_path.unpack().clone());
    //             }
    //         }
    //     }
    // }

    let mut total_failure_count = 0;

    let failure_count = evaluate_test_module(path)?;
    total_failure_count += failure_count;

    // #todo also keep passed and total statistics.

    if total_failure_count > 0 {
        println!("\nFailed: {total_failure_count}");
    }

    Ok(())
}

use std::{
    collections::HashSet,
    path::Path,
    sync::{Arc, RwLock},
};

use clap::ArgMatches;
use glob::Pattern;
use tan::{
    context::Context,
    eval::{invoke_func, util::eval_module},
    expr::{format_value, Expr},
    util::standard_names::PROFILE,
};

use crate::util::{
    ansi::{bold, green, red},
    fs::filter_walk_dir,
    report::report_errors,
};

// cargo r -- test tests/fixtures/test-fixture

// #todo
// Better report the results:
//
// test /path/to/test
// - test-one OK
// - test-another OK
// test /path/to/another/test
// - test-function OK
// ...

// #todo recursively scan for test files!
// #todo add command-line option to disable recursive scan.

// #todo add unit tests to verify error logging, etc.
// #todo use a different name than test: spec, conformance, something else?
// #todo *.test.tan files should be ignored on non-test-profile runs.
// #todo attach description annotations to unit-test functions to be printed when the tests run, is this really useful?

// #todo reuse run cmd infrastructure.

// #todo Consider using the `glob` crate to implement the fs walk.

// #todo find better name.
// #todo extract to tan-analysis
// #todo how to ensure tests are not run multiple times with module interdependencies, will need some work.
// #todo should return a tree? or at least have a version with a tree.
#[allow(dead_code)]
pub fn compute_module_paths(
    base_path: impl AsRef<Path>,
    file_pattern: &Option<Pattern>,
) -> Result<Vec<String>, std::io::Error> {
    let predicate = |p: &str| (!p.contains("/.git/")) && p.ends_with(".test.tan");

    // #todo Rename path to base_path or something.
    let paths = filter_walk_dir(base_path.as_ref(), &predicate)?;
    let mut path_set: HashSet<String> = HashSet::new();
    for path in paths {
        if let Some(pattern) = file_pattern {
            if !pattern.matches(&path) {
                continue;
            }
        }
        // #insight Computes the unique modules (directories), also reads non-test files.
        path_set.insert(
            Path::new(&path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }
    Ok(path_set.into_iter().collect())
}

fn evaluate_test_module(path: &str, file_pattern: &Option<Pattern>) -> anyhow::Result<usize> {
    let path = Path::new(path);

    // #todo handle general URLs, not only file://

    let path = std::fs::canonicalize(path)?;
    let path = format!("file://{}", path.to_string_lossy());

    let mut context = Context::new();

    let test_failures: Arc<RwLock<Vec<Expr>>> = Arc::new(RwLock::new(Vec::new()));

    context
        .dynamic_scope
        .insert("*test-failures*", Expr::Array(test_failures.clone()));

    // #insight don't set CURRENT_MODULE_PATH, it will be set in eval_module.
    // #insight don't set CURRENT_FILE_PATH, it will be set in eval_module and invoke_func.

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
                if let Some(pattern) = file_pattern {
                    if !pattern.matches(file_path) {
                        continue;
                    }
                }

                test_failures.write().unwrap().clear();
                print!("test `{name}` in `{file_path}` ");

                // #todo No need for manual current-file-path handling, put in scope!
                // let old_current_file_path = context.top_scope.get(CURRENT_FILE_PATH);
                // context
                //     .top_scope
                //     .insert(CURRENT_FILE_PATH, Expr::string(file_path));
                // #insight No need to maintain a stack.
                // context
                //     .scope
                //     .insert(CURRENT_FILE_PATH, Expr::string(file_path));

                // #todo will need to pass arguments by ref.
                let result = invoke_func(value, Vec::new(), &mut context);

                if let Err(error) = result {
                    report_errors(&[error], None);
                    // #todo flag the error here
                    println!("{}", bold(red("FAIL")));
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

                // #insight No need to maintain a stack.
                // if let Some(old_current_file_path) = old_current_file_path {
                //     // #insight we should revert the previous current file, in case of 'use'
                //     context
                //         .top_scope
                //         .insert(CURRENT_FILE_PATH, old_current_file_path.unpack().clone());
                // }
            }
        }
    }

    Ok(total_failure_count)
}

pub fn handle_test(test_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = test_matches.get_one("PATH").unwrap();
    let file_glob: Option<&String> = test_matches.get_one("FILE");

    // #todo Make sure the file_glob starts with the path.
    let file_pattern =
        file_glob.map(|glob| glob::Pattern::new(glob).expect("file glob pattern is valid"));

    // #todo Extract as helper function, in analysis.
    let paths = compute_module_paths(path, &file_pattern)?;

    let mut total_failure_count = 0;

    for path in paths {
        let failure_count = evaluate_test_module(&path, &file_pattern)?;
        total_failure_count += failure_count;
    }

    // #todo also keep passed and total statistics.

    if total_failure_count > 0 {
        println!("\nFailed: {total_failure_count}");
    }

    Ok(())
}

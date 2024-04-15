use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use clap::ArgMatches;
use tan::{
    context::Context,
    eval::{invoke_func, util::eval_module},
    expr::{format_value, Expr},
};

// cargo r -- test tests/fixtures/test-fixture

// #todo use a different name than test: spec, conformance, something else?

// // #todo apply some ordering
// // #todo follow symlinks
// // #todo a better name, filter_test_file_paths?
// fn compute_test_file_paths(path: &str) -> Vec<PathBuf> {
//     let mut paths = Vec::new();
//     for entry in WalkDir::new(path) {
//         let entry = entry.unwrap();
//         // #todo there must be a better way.
//         if entry.path().display().to_string().ends_with(".test.tan") {
//             paths.push(entry.path().into());
//         }
//     }
//     paths
// }

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

    // #todo proper error-handling needed here.
    let result = eval_module(path, &mut context, false);

    let expr = result.unwrap();
    let Expr::Module(module) = expr.unpack() else {
        // #todo proper error-handling.
        panic!("error");
    };

    for (name, value) in module.scope.bindings.read().unwrap().iter() {
        // #insight #temp test-case methods start with test.
        if name.starts_with("test") {
            if let Expr::Func(_, _, _, filename) = value.unpack() {
                test_failures.write().unwrap().clear();
                print!("test `{name}` in `{filename}` ");
                invoke_func(value, &Vec::new(), &mut context)?;
                let failure_count = test_failures.write().unwrap().len();
                if failure_count > 0 {
                    println!("FAIL");
                    for failure in test_failures.read().unwrap().iter() {
                        println!("{}", format_value(failure));
                    }
                } else {
                    println!("OK");
                }
            }
        }
    }

    Ok(())
}

// pub fn handle_test_old(test_matches: &ArgMatches) -> anyhow::Result<()> {
//     let path: &String = test_matches.get_one("PATH").unwrap();

//     let test_file_paths = compute_test_file_paths(path);

//     for path in test_file_paths {
//         print!("test {}", path.display());
//         let result = eval_file(path.display().to_string());

//         // let path = path.display().to_string();
//         // let input = std::fs::read_to_string(path).unwrap();
//         // let mut context = Context::new();
//         // #todo inject *testing-context* or *testing-session*
//         // context.dynamic_scope.insert(name, value);
//         // let result = eval_string(input, &mut context);

//         // #todo reuse tan functionality?

//         // #todo ansi colors needed here.
//         if result.is_ok() {
//             println!(" ..pass");
//         } else {
//             // #todo detailed reporting required!
//             println!(" ..fail");
//         }
//     }

//     Ok(())
// }

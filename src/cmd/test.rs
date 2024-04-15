use std::path::PathBuf;

use clap::ArgMatches;
use walkdir::WalkDir;

// #todo
// it's probably easier to implement this with Tan?
// scan files
// inject *testing-context*
// eval the files
// dump reported errors

use crate::util::eval_file;

// cargo r -- test tests/fixtures/test-fixture

// #todo use a different name than test: spec, conformance, something else?

// #todo apply some ordering
// #todo follow symlinks
// #todo a better name, filter_test_file_paths?
fn compute_test_file_paths(path: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        // #todo there must be a better way.
        if entry.path().display().to_string().ends_with(".test.tan") {
            paths.push(entry.path().into());
        }
    }
    paths
}

pub fn handle_test(test_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = test_matches.get_one("PATH").unwrap();

    let test_file_paths = compute_test_file_paths(path);

    for path in test_file_paths {
        print!("test {}", path.display());
        let result = eval_file(path.display().to_string());

        // let path = path.display().to_string();
        // let input = std::fs::read_to_string(path).unwrap();
        // let mut context = Context::new();
        // #todo inject *testing-context* or *testing-session*
        // context.dynamic_scope.insert(name, value);
        // let result = eval_string(input, &mut context);

        // #todo reuse tan functionality?

        // #todo ansi colors needed here.
        if result.is_ok() {
            println!(" ..pass");
        } else {
            // #todo detailed reporting required!
            println!(" ..fail");
        }
    }

    Ok(())
}

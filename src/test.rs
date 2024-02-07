use std::path::PathBuf;

use clap::ArgMatches;
use walkdir::WalkDir;

// cargo r -- test tests/fixtures/test-fixture

// #todo use a different name than test: spec, conformance, something else?

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
        // #todo ansi colors needed here.
        println!(" ..ok");
    }

    Ok(())
}

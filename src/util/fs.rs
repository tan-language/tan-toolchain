use std::{fs, path::Path};

// #todo add unit test, move to a util package.
// #todo find a better name than walk_dir.
// #todo accept separate dir and file predicate?
// #todo accept separate include and scan predicate?
// #todo the predicate should accept a Path/PathBuf?
pub fn filter_walk_dir<P>(dir_path: &Path, predicate: &P) -> Result<Vec<String>, std::io::Error>
where
    P: Fn(&str) -> bool,
{
    let mut tree = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry_path = entry?.path();
        // #todo investigate this unwrap.
        let entry_path_str = entry_path.to_str().unwrap().to_string();

        if entry_path.is_dir() {
            if predicate(&entry_path_str) {
                tree.push(format!("{entry_path_str}/"));
            }
            tree.append(&mut filter_walk_dir(&entry_path, predicate)?);
        } else if predicate(&entry_path_str) {
            tree.push(entry_path_str);
        }
    }

    Ok(tree)
}

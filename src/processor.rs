use std::fs;

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}
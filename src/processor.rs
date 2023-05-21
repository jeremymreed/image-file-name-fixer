use std::fs;

pub fn process_path(path: &String) {
    let abs_path: String = fs::canonicalize(path).unwrap().display().to_string();

    match fs::metadata(abs_path) {
        Ok(attrs) => println!("attrs: {:?}", attrs),
        Err(error) => eprint!("error: {}", error),
    }
}
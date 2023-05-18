use std::fs;

fn check_attrs(abs_path: &String) {
    match fs::metadata(abs_path) {
        Ok(attrs) => println!("attrs: {:?}", attrs),
        Err(error) => eprint!("error: {}", error),
    }
}

pub fn is_valid_path(abs_path: &String) -> Result<bool, String> {
    println!("Input path: {}", abs_path);

    check_attrs(abs_path);

    Err(String::from("Not implemented yet!"))
}
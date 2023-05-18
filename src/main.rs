use image::GenericImageView;
use shadow_rs::shadow;

mod arg_parser;
mod config;
mod check_path;

shadow!(build);

fn main() {
    let config: config::Config = arg_parser::parse_args();

    match check_path::is_valid_path(&config.file_name) {
        Ok(result) => if result {
            println!("Valid path!");
        } else {
            println!("Invalid path!");
        },
        Err(message) => println!("Got error: {}", message),
    }

    let img = image::open(config.file_name).unwrap();

    println!("Dimensions: {:?}", img.dimensions());
}

use image::GenericImageView;

mod arg_parser;
mod config;

fn main() {
    let config: config::Config = arg_parser::parse_args();

    let img = image::open(config.file_name).unwrap();

    println!("Dimensions: {:?}", img.dimensions());
}

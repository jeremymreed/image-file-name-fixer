use image::GenericImageView;
use image::ImageFormat;
use image::io::Reader;
use shadow_rs::shadow;

mod arg_parser;
mod config;
mod processor;

shadow!(build);

fn main() {
    let config: config::Config = arg_parser::parse_args();

    processor::process_path(&config.absolute_path);

    //let img = image::open(config.file_name).unwrap();

    let reader = Reader::open(config.absolute_path).unwrap()
        .with_guessed_format()
        .expect("Failed to open image file");

    println!("Format: {:?}", reader.format());
    let format = match reader.format() {
        Some(format) => format,
        None => panic!("No format"),
    };

    let test = match format {
        ImageFormat::Png => String::from("png"),
        ImageFormat::Jpeg => String::from("jpeg"),
        ImageFormat::Gif => String::from("gif"),
        ImageFormat::WebP => String::from("webp"),
        _ => panic!("Unspported format"),
    };

    println!("Format: {}", test);

    let img = reader.decode().expect("Failed to read image");

    println!("Dimensions: {:?}", img.dimensions());
}

use std::fs;
use image::GenericImageView;
use image::ImageFormat;
use image::io::Reader;

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}

pub fn process_file(absolute_path: &String) {
    let reader = Reader::open(absolute_path).unwrap()
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
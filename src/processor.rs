use std::fs;
use image::GenericImageView;
use image::ImageFormat;
use image::io::Reader;
use crate::image_data;

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}

pub fn process_file(absolute_path: &String) {
    let reader = Reader::open(absolute_path).unwrap()
        .with_guessed_format()
        .expect("Failed to open image file");

    println!("Format: {:?}", reader.format());
    let raw_format = match reader.format() {
        Some(format) => format,
        None => panic!("Unknown format"),
    };

    let format = match raw_format {
        ImageFormat::Png => String::from("png"),
        ImageFormat::Jpeg => String::from("jpeg"),
        ImageFormat::Gif => String::from("gif"),
        ImageFormat::WebP => String::from("webp"),
        _ => panic!("Unspported format"),
    };

    println!("Format: {}", format);

    let img = reader.decode().expect("Failed to read image");

    println!("Dimensions: {:?}", img.dimensions());

    let image_data = image_data::ImageData {
        absolute_path: String::from(absolute_path),
        format: format,
        width: img.width(),
        height: img.height(),
    };

    println!("image_data: {:?}", image_data);
}
use crate::image_data;
use image::io::Reader;
use image::GenericImageView;
use image::ImageFormat;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref PATTERN: Regex = Regex::new("([-]?[0-9]+[×xX][0-9]+|[-]?[0-9a-fA-F]{64})").unwrap();
}

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}

pub fn fix_file_name(image_data: &mut image_data::ImageData) {
    let mut result: PathBuf = PathBuf::from(image_data.absolute_path.clone());

    let file_name = result.file_stem().unwrap().to_str().unwrap();

    let test = PATTERN.replace_all(file_name, "");

    result.set_file_name(format!(
        "{}-{}x{}.{}",
        test, image_data.width, image_data.height, image_data.format
    ));

    image_data.final_absolute_path = result.display().to_string();
}

pub fn copy_file(image_data: &image_data::ImageData) {
    fs::copy(
        image_data.absolute_path.clone(),
        image_data.final_absolute_path.clone(),
    )
    .expect("Failed to rename file");
}

pub fn process_file(absolute_path: &String) {
    let reader = Reader::open(absolute_path)
        .unwrap()
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

    let img = reader.decode().expect("Failed to read image");

    let mut image_data = image_data::ImageData {
        absolute_path: String::from(absolute_path),
        final_absolute_path: String::from(""),
        format: format,
        width: img.width(),
        height: img.height(),
    };

    fix_file_name(&mut image_data);

    copy_file(&image_data);
}

use crate::image_data;
use crate::config;
use image::io::Reader;
use image::GenericImageView;
use sha256;
use image::ImageFormat;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use crate::config::Config;

lazy_static! {
    static ref PATTERN: Regex = Regex::new("([-]?[0-9]+[×xX][0-9]+|[-]?[0-9a-fA-F]{64})").unwrap();
}

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}

pub fn generate_hash(image_data: &mut image_data::ImageData) {
    image_data.hash = sha256::try_digest(Path::new(&image_data.absolute_path)).unwrap();
}

pub fn fix_file_name(config: &config::Config, image_data: &mut image_data::ImageData) {
    let mut final_file_path: PathBuf = PathBuf::from(image_data.absolute_path.clone());

    let file_name = final_file_path.file_stem().unwrap().to_str().unwrap();

    let clean_file_name = PATTERN.replace_all(file_name, "");

    if config.should_hash {
        final_file_path.set_file_name(format!(
            "{}-{}-{}x{}.{}",
            clean_file_name, image_data.hash, image_data.width, image_data.height, image_data.format
        ));
    } else {
        final_file_path.set_file_name(format!(
            "{}-{}x{}.{}",
            clean_file_name, image_data.width, image_data.height, image_data.format
        ));
    }

    image_data.final_absolute_path = final_file_path.display().to_string();
}

pub fn copy_file(image_data: &image_data::ImageData) {
    fs::copy(
        image_data.absolute_path.clone(),
        image_data.final_absolute_path.clone(),
    )
    .expect("Failed to copy file");
}

pub fn move_file(image_data: &image_data::ImageData) {
    fs::rename(
        image_data.absolute_path.clone(),
        image_data.final_absolute_path.clone(),
    ).expect("Failed to rename file");
}

pub fn process_file(config: &Config) {
    let reader = Reader::open(&config.absolute_path)
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
        absolute_path: String::from(&config.absolute_path),
        final_absolute_path: String::from(""),
        format: format,
        width: img.width(),
        height: img.height(),
        hash: String::from(""),
    };

    generate_hash(&mut image_data);

    fix_file_name(&config, &mut image_data);

    if config.move_files {
        move_file(&image_data);
    } else {
        copy_file(&image_data);
    }
}

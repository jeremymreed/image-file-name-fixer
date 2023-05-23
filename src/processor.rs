use crate::config;
use crate::config::Config;
use crate::image_data;
use image::io::Reader;
use image::ImageFormat;
use lazy_static::lazy_static;
use regex::Regex;
use sha256;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

lazy_static! {
    static ref PATTERN: Regex = Regex::new("([-]?[0-9]+[×xX][0-9]+|[-]?[0-9a-fA-F]{64})").unwrap();
}

pub fn process_path(path: &String) -> String {
    let absolute_path: String = fs::canonicalize(path).unwrap().display().to_string();

    absolute_path
}

pub fn generate_hash(image_data: &image_data::ImageData) -> String {
    sha256::try_digest(Path::new(&image_data.absolute_path)).unwrap()
}

pub fn fix_file_name(config: &config::Config, image_data: &image_data::ImageData) -> String {
    let mut final_file_path: PathBuf = PathBuf::from(image_data.absolute_path.clone());

    let file_name = final_file_path.file_stem().unwrap().to_str().unwrap();

    let clean_file_name = PATTERN.replace_all(file_name, "");

    if config.should_hash {
        final_file_path.set_file_name(format!(
            "{}-{}-{}x{}.{}",
            clean_file_name,
            image_data.hash,
            image_data.width,
            image_data.height,
            image_data.format
        ));
    } else {
        final_file_path.set_file_name(format!(
            "{}-{}x{}.{}",
            clean_file_name, image_data.width, image_data.height, image_data.format
        ));
    }

    final_file_path.display().to_string()
}

pub fn print_output(prefix: String, image_data: &image_data::ImageData) {
    println!(
        "{}: {} -> {}",
        prefix, image_data.absolute_path, image_data.final_absolute_path
    );
}

pub fn copy_file(config: &Config, image_data: &image_data::ImageData) {
    if !config.dry_run {
        fs::copy(
            image_data.absolute_path.clone(),
            image_data.final_absolute_path.clone(),
        )
        .expect("Failed to copy file");
    }
    print_output(String::from("Copying"), &image_data);
}

pub fn move_file(config: &Config, image_data: &image_data::ImageData) {
    if !config.dry_run {
        fs::rename(
            image_data.absolute_path.clone(),
            image_data.final_absolute_path.clone(),
        )
        .expect("Failed to move file");
    }
    print_output(String::from("Moving"), &image_data);
}

pub fn process(config: &Config, absolute_path: &String) {
    let metadata = fs::metadata(absolute_path).unwrap();

    if metadata.is_dir() {
        process_directory(&config, absolute_path);
    } else {
        process_file(&config, absolute_path);
    }
}

pub fn process_file(config: &Config, absolute_path: &String) {
    let reader = Reader::open(absolute_path)
        .unwrap()
        .with_guessed_format()
        .expect("Failed to open image file");

    let raw_format = match reader.format() {
        Some(format) => format,
        None => {
            // Skip over file.
            println!("{}: Unrecognized format", absolute_path);
            return;
        }
    };

    let format = match raw_format {
        ImageFormat::Png => String::from("png"),
        ImageFormat::Jpeg => String::from("jpeg"),
        ImageFormat::Gif => String::from("gif"),
        ImageFormat::WebP => String::from("webp"),
        _ => {
            // Skip over file.
            println!("{}: Unsupported format", absolute_path);
            return;
        }
    };

    let img = reader.decode().expect("Failed to read image");

    let mut image_data = image_data::ImageData {
        absolute_path: String::from(absolute_path),
        final_absolute_path: String::from(""),
        format: format,
        width: img.width(),
        height: img.height(),
        hash: String::from(""),
    };

    image_data.hash = generate_hash(&image_data);

    image_data.final_absolute_path = fix_file_name(&config, &image_data);

    if config.move_files {
        move_file(&config, &image_data);
    } else {
        copy_file(&config, &image_data);
    }
}

pub fn process_directory(config: &Config, absolute_path: &String) {
    let paths = fs::read_dir(absolute_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        process(&config, &path.display().to_string());
    }
}

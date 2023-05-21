use clap::{Arg, Command};
use lazy_static::lazy_static;
use crate::processor;
use crate::config;
use crate::build;

pub fn parse_args() -> config::Config {
    // Build our version string.
    lazy_static! {
        static ref PKG_VERSION: String = format!("v{}", build::PKG_VERSION);
    }

    // let pkg_version = concat!("v", clap::crate_version!());
    let matches = Command::new("Image File Name Fixer")
        .version(PKG_VERSION.as_str())
        .about(clap::crate_description!())
        .arg(Arg::new("raw_path"))
        .get_matches();

    println!("raw_path: {:?}", matches.get_one::<String>("raw_path"));

    let raw_path = match matches.get_one::<String>("raw_path") {
        Some(raw_path) => raw_path,
        None => panic!("Must give a valid path!"),
    };

    // If the path is invalid, panic.
    let absolute_path = processor::process_path(raw_path);

    println!("absolute_path: {:?}", absolute_path);

    config::Config {
        absolute_path: absolute_path.clone(),
    }
}
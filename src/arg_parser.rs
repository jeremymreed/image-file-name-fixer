use std::fs;
use clap::{Arg, Command};
use lazy_static::lazy_static;
use crate::config;
use crate::build;

pub fn parse_args() -> config::Config {
    lazy_static! {
        static ref PKG_VERSION: String = format!("v{}", build::PKG_VERSION);
    }

    // let pkg_version = concat!("v", clap::crate_version!());
    let matches = Command::new("Image File Name Fixer")
        .version(PKG_VERSION.as_str())
        .about(clap::crate_description!())
        .arg(Arg::new("file_name"))
        .get_matches();

    println!("file_name: {:?}", matches.get_one::<String>("file_name"));

    let file_name = match matches.get_one::<String>("file_name") {
        Some(file_name) => file_name,
        None => panic!("Must give a file name!"),
    };

    let absolute_path = fs::canonicalize(file_name).unwrap().display().to_string();

    println!("absolute_path: {:?}", absolute_path);

    config::Config {
        file_name: absolute_path.clone(),
    }
}
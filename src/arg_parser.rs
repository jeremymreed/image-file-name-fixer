use clap::{Arg, Command};
use crate::config;

pub fn parse_args() -> config::Config {
    let pkg_version = concat!("v", clap::crate_version!());
    let matches = Command::new("Image File Name Fixer")
        .version(pkg_version)
        .about(clap::crate_description!())
        .arg(Arg::new("file_name"))
        .get_matches();

    println!("file_name: {:?}", matches.get_one::<String>("file_name"));

    let file_name = match matches.get_one::<String>("file_name") {
        Some(file_name) => file_name,
        None => panic!("Must give a file name!"),
    };

    config::Config {
        file_name: file_name.clone(),
    }
}
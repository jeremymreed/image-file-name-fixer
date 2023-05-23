use crate::build;
use crate::config;
use crate::processor;
use clap::{Arg, ArgAction, Command};
use lazy_static::lazy_static;

pub fn parse_args() -> config::Config {
    // Build our version string.
    lazy_static! {
        static ref PKG_VERSION: String = format!("v{}", build::PKG_VERSION);
    }

    // let pkg_version = concat!("v", clap::crate_version!());
    let matches = Command::new("Image File Name Fixer")
        .version(PKG_VERSION.as_str())
        .about(clap::crate_description!())
        .arg(
            Arg::new("move")
                .short('m')
                .long("move")
                .action(ArgAction::SetTrue)
                .help("Move the files instead of copying them."),
        )
        .arg(
            Arg::new("hash")
                .long("hash")
                .action(ArgAction::SetTrue)
                .help("Calculate and include sha256 hashes in file names."),
        )
        .arg(Arg::new("raw_path"))
        .get_matches();

    println!("raw_path: {:?}", matches.get_one::<String>("raw_path"));

    let raw_path = match matches.get_one::<String>("raw_path") {
        Some(raw_path) => raw_path,
        None => panic!("Must give a valid path!"),
    };

    let move_files = match matches.get_one::<bool>("move") {
        Some(move_files) => move_files,
        None => panic!("Got some garbage value!"),
    };

    let should_hash = match matches.get_one::<bool>("hash") {
        Some(should_hash) => should_hash,
        None => panic!("Got some garbage value!"),
    };

    // If the path is invalid, panic.
    let absolute_path = processor::process_path(raw_path);

    config::Config {
        starting_absolute_path: absolute_path.clone(),
        move_files: *move_files,
        should_hash: *should_hash,
    }
}

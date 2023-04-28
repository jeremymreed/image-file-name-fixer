use clap::{Arg, Command};

pub fn parse_args() {
    let pkg_version = concat!("v", clap::crate_version!());
    let matches = Command::new("Image File Name Fixer")
        .version(pkg_version)
        .about("Normalizes image file names!")
        .arg(Arg::new("file_name"))
        .get_matches();

    println!("matches: {:?}", matches);
}
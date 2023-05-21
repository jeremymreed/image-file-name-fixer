use shadow_rs::shadow;
mod arg_parser;
mod config;
mod processor;

shadow!(build);

fn main() {
    let config: config::Config = arg_parser::parse_args();

    processor::process_path(&config.absolute_path);

    //let img = image::open(config.file_name).unwrap();

    processor::process_file(&config.absolute_path);
}

use shadow_rs::shadow;
mod arg_parser;
mod config;
mod image_data;
mod processor;

shadow!(build);

fn main() {
    let config: config::Config = arg_parser::parse_args();

    processor::process_path(&config.absolute_path);

    processor::process_file(&config.absolute_path);
}

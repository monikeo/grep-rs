mod cli;
mod file;
mod search;
mod recursive;
mod model;
use model::Config;
use search::search_lines;

use grep_rs::*;
use grep_rs::cli::Args;

use std::path::PathBuf;

fn run(args: Args) {
    let query = args.get_pattern();

    if args.get_file().is_some() && args.get_files().is_some() {
        let message = "You cannot specify both `--file` and `--files` options at the same time.";  
        error_exit(message);
    }

    let files = get_file_paths(&args);
    let config = build_config(&args, query, files);
    let result = search_lines(&config);

    for (key, values) in result {
        if values.is_ok() {
            println!("|  {}", key.to_string_lossy());
            for value in values.unwrap() {
                println!("|  {}", value);
            }
        }
        println!();
    }
}

fn build_config(args: &Args, query: &str, files: Vec<PathBuf> ) -> Config {
    let mut config = Config::new(query, files);
    config.set_recursive(args.get_recursive()) ;
    config.set_ignore_case(args.get_ignore_case());
    config.set_line_number(args.get_line_number());
    config.set_invert_match(args.get_invert_match());
    config
}

fn main() {
    let args = Args::parse_args();
    run(args);
}

mod cli;
mod file;
mod model;
mod recursive;
mod search;
use model::Config;
use search::search_lines;
use file::FileError;

use grep_rs::cli::Args;
use grep_rs::*;

use std::path::PathBuf;
use std::collections::HashMap;

fn run(args: Args) -> Result<(), String>{
    let query = args.get_pattern();

    if args.get_file().is_some() && args.get_files().is_some() {
        let message = "You cannot specify both `--file` and `--files` options at the same time.";
        return Err(message.to_string());
    }

    let files = get_file_paths(&args)?;
    let config = build_config(&args, query, files);
    let results = search_lines(&config);

    display_result(results);
    Ok(())
}


/// Displays the search results.
///
/// # Arguments
/// * `results` - The search results as a `HashMap`.
fn display_result(results: HashMap<PathBuf, Result<Vec<String>, FileError>>) {
    for (path, result) in results {
        match result {
            Ok(lines) => {
                println!("File: {}", path.display());
                for line in lines {
                    println!("{}", line);
                }
                println!();
            },
            Err(err) => {
                eprintln!("[-] ERROR: {}", err);
            }
        }
    }
}

/// Constructs the configuration for the search.
///
/// # Arguments
/// * `args` - Parsed CLI arguments.
/// * `query` - The search query.
/// * `files` - List of file paths.
///
/// # Returns
/// A `Config` object.
fn build_config(args: &Args, query: &str, files: Vec<PathBuf>) -> Config {
    let mut config = Config::new(query, files);
    config.set_recursive(args.get_recursive());
    config.set_ignore_case(args.get_ignore_case());
    config.set_line_number(args.get_line_number());
    config.set_invert_match(args.get_invert_match());
    config
}


/// Main function to handle argument parsing and run the application.
fn main() {
    let args = Args::parse_args();
    if let Err(err) = run(args) {
        error_exit(&err);
    }
}

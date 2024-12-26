mod cli;
mod file;
mod search;
mod recursive;
mod model;
use file::{FileError, FileHandling};
use cli::Args;
use recursive::find_files_recursively;

use grep_rs::*;

use clap::Parser;

fn main() {
    let args = Args::parse_args();
    let query = args.get_pattern();

    if args.get_file().is_some() && args.get_files().is_some() {
        let message = "You cannot specify both `--file` and `--files` options at the same time.";  
        error_exit(message);
    }


    let files = if args.get_recursive() && args.get_directory().is_some() {
        let directory = args.get_directory().unwrap();
        if FileHandling::is_dir(directory) {
            find_files_recursively(directory)
        } else {
            Vec::new()
        }
    } else if let Some(file) = args.get_file() {
        vec![file.into()]
    } else if let Some(files) = args.get_files(){
        files.into_iter().map(Into::into).collect()
    } else {
        let message = "No file or files specified. Use `--file` or `--files` to provide input.";
        error_exit(message);
        Vec::new()
    };



}

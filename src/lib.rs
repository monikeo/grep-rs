pub mod cli;
pub mod file;
pub mod model;
pub mod search;
pub mod recursive;
use cli::Args;
use file::FileHandling;
use recursive::find_files_recursively;

use std::process;
use std::path::PathBuf;

pub fn error_exit(message: &str) {
    println!(" [-] ERROR: {}", message);
    process::exit(0x0100);
}

pub fn get_file_paths(args: &Args) -> Vec<PathBuf>{
    if args.get_recursive() && args.get_directory().is_some() {
        if args.get_file().is_some() || args.get_files().is_some() {
            let message = "You cannot specify both file search and recursive at the same time.";
            error_exit(message);
        }
        let directory = args.get_directory().unwrap();
        if FileHandling::is_dir(directory) {
            find_files_recursively(directory)
        } else {
            Vec::new()
        }
    } else if let Some(file) = args.get_file() {
        vec![file.into()]
    } else if let Some(files) = args.get_files() {
        files.into_iter().map(Into::into).collect()
    } else {
        let message = "No file or files specified. Use `--file` or `--files` to provide input.";
        error_exit(message);
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}

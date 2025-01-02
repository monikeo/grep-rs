pub mod cli;
pub mod file;
pub mod model;
pub mod recursive;
pub mod search;
use cli::Args;
use file::{FileError, FileHandling};
use recursive::find_files_recursively;

use std::path::PathBuf;
use std::process;

pub fn error_exit(message: &str) {
    println!(" [-] ERROR: {}", message);
    process::exit(0x0100);
}

/// Retrieves file paths based on CLI arguments.
///
/// # Arguments
/// * `args` - Parsed CLI arguments.
///
/// # Returns
/// A vector of file paths or an error message.
pub fn get_file_paths(args: &Args) -> Result<Vec<PathBuf>, String> {
    if args.get_recursive() && args.get_directory().is_some() {
        handle_recursive(args)
    } else if let Some(file) = args.get_file() {
        Ok(vec![file.into()])
    } else if let Some(files) = args.get_files() {
        Ok(files.into_iter().map(Into::into).collect())
    } else {
        Err("No file or files specified. Use `--file` or `--files` to provide input.".into())
    }
}

/// Handles recursive file searching logic.
///
/// # Arguments
/// * `args` - Parsed CLI arguments.
///
/// # Returns
/// A vector of file paths or an error message.
pub fn handle_recursive(args: &Args) -> Result<Vec<PathBuf>, String> {
    if args.get_file().is_some() || args.get_files().is_some() {
        return Err("You cannot specify both file search and recursive at the same time.".into())
    }
    if let Some(directory) = args.get_directory() {
        if FileHandling::is_dir(directory) {
            Ok(find_files_recursively(directory))
        } else {
            Err(format!("Provided directory '{}' is not valid.", directory))
        }
    } else {
        Err("Recursive search requires a directory. Use `--directory` to specify one.".into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

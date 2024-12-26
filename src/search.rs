use crate::model::Config;
use crate::file::{
    FileError,
    FileHandling
};
use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;

/// Searches for lines matching the query in a specific file.
///
/// # Arguments
/// - `regex`: The compiled regular expression.
/// - `path`: Path to the file to be searched.
/// - `line_number`: Whether to include line numbers in the output.
/// - `invert_match`: If true, selects non-matching lines instead.
///
/// # Returns
/// - `Ok(Vec<String>)`: Matching lines from the file.
/// - `Err(FileError)`: Any errors encountered while reading the file.
pub fn search_lines_with_file(
    regex: &Regex, 
    path: &PathBuf, 
    line_number: bool, 
    invert_match: bool
) -> Result<Vec<String>, FileError> {
    let lines = FileHandling::read_lines(path)?;
    let matched_lines = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| regex.is_match(line) != invert_match)
        .map(|(index, line)| {
            if line_number {
                format!("{}: {}", index + 1, line)
            } else {
                line.to_string()
            }
        }).collect();

    Ok(matched_lines)
}

/// Searches for lines matching the query across multiple files.
///
/// # Arguments
/// - `config`: The search configuration containing query, file paths, and options.
///
/// # Returns
/// - `HashMap<PathBuf, Result<Vec<String>, FileError>>`: Map of file paths to search results or errors.
pub fn search_lines(config: &Config) -> HashMap<PathBuf, Result<Vec<String>, FileError>> {
    let regex = if config.get_ignore_case() {
        Regex::new(&format!("(?i){}", config.get_query())).unwrap()
    } else {
        Regex::new(config.get_query()).unwrap()
    };

    let path_bufs = config.get_file_paths();
    
    let mut results: HashMap<PathBuf, Result<Vec<String>, FileError>> = HashMap::new();

    for path in path_bufs {
        let result = search_lines_with_file(
            &regex, 
            path, 
            config.get_line_number(), 
            config.get_invert_match()
        );
        results.insert(path.clone(), result);
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_search_lines() {
        let file_path = "./src/main.rs";
        let path = vec![Path::new(file_path).to_path_buf()];
        let pattern = "fn";
        let config = Config::new(pattern, path);
        let result = search_lines(&config);
        assert!(result.get(Path::new(file_path)).unwrap().is_ok());
        println!("{:#?}", result);
    }

    #[test]
    fn test_search_lines_show_line_number() {
        let file_path = "./src/main.rs";
        let path = vec![Path::new(file_path).to_path_buf()];
        let pattern = "mod";
        let mut config = Config::new(pattern, path);
        config.set_line_number(true);
        let result = search_lines(&config);
        assert!(result.get(Path::new(file_path)).unwrap().is_ok());
        println!("{:#?}", result);
    }
    /*
    #[test]
    fn test_search_lines() {
        let file_path = "./src/main.rs";
        let pattern = "fn";
        let config = Config::new(pattern,file_path);
        let result = search_lines(&config);
        assert!(result.is_ok());
        println!("{:#?}", result);
    }

    #[test]
    fn test_search_lines_show_line_number() {
        let file_path = "./src/main.rs";
        let pattern = "mod";
        let mut config = Config::new(pattern, file_path);
        config.set_line_number(true);
        let result = search_lines(&config);
        assert!(result.is_ok());
        println!("{:#?}", result);
    }
    */
}

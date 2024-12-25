use crate::file::{
    FileError,
    FileHandling
};
use std::path::PathBuf;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    line_number: bool,
    recursive: bool,
    invert_match: bool,
}

impl Config {
    pub fn new(query: &str, file_path: &str) -> Self {
        Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: false,
            line_number: false,
            recursive: false,
            invert_match: false
        }
    }

    pub fn set_ignore_case(&mut self, flag: bool) {
        self.ignore_case = flag;
    }
    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn set_line_number(&mut self, flag: bool) {
        self.line_number = flag;
    }
    pub fn get_line_number(&self) -> bool {
        self.line_number
    }

    pub fn set_recursive(&mut self, flag: bool) {
        self.recursive = flag;
    }
    pub fn get_recursive(&self) -> bool {
        self.recursive
    }
    
    pub fn invert_match(&mut self, flag: bool) {
        self.invert_match = flag;
    }
    pub fn get_invert_match(&self) -> bool {
        self.invert_match
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn validate_path(&self) -> Result<PathBuf, FileError> {
        FileHandling::validate_path(&self.file_path)
    }
    pub fn read_lines(&self) -> Result<Vec<String>, FileError> {
        FileHandling::read_lines(&self.file_path)
    }
    pub fn read_multiple_files() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let query = "test_query";
        let file_path = "./src/main.rs";
        let config = Config::new(query, file_path);
        assert_eq!(query, config.get_query());
        assert_eq!(file_path, config.get_file_path());

        let validate_path = config.validate_path();
        let read_lines = config.read_lines();
        assert!(validate_path.is_ok());
        assert!(read_lines.is_ok());
    }

    #[test]
    fn test_config1() {
        let query = "test_query";
        let file_path = "./src/sjdkfjsdofjsdif.rs";
        let config = Config::new(query, file_path);

        let validate_path = config.validate_path();
        let read_lines = config.read_lines();
        assert!(!validate_path.is_ok());
        assert!(!read_lines.is_ok());
    }
}

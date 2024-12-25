use crate::file::{
    FileError,
    FileHandling
};
use std::path::{
    Path, PathBuf
};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(query: &str, file_path: &str) -> Self {
        Self {
            query: query.to_string(),
            file_path: file_path.to_string()
        }
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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Access denied to file: {0}")]
    AccessDenied(String),
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error)
}

pub mod FileHandling {
    use std::path::{
        Path,
        PathBuf
    };
    use std::fs;
    use super::*;

    pub fn validate_path(path: &str) -> Result<PathBuf, FileError> {
        let path = std::path::Path::new(path);
        if !path.exists() {
            return Err(FileError::NotFound(path.to_string_lossy().to_string()));
        }
        if !path.is_file() {
            return Err(FileError::InvalidPath(path.to_string_lossy().to_string()));
        }
        let file = fs::canonicalize(path);
        match file {
            Ok(path_buf) => Ok(path_buf),
            Err(err) => Err(FileError::IoError(err))
        }
        
    }

    pub fn read_file(path: &str) -> Result<String, FileError> {
        let path = self::validate_path(path)?;
        fs::read_to_string(path).map_err(FileError::from)
    }

    pub fn read_lines(path: &str) -> Result<Vec<String>, FileError> {
        let content = self::read_file(path)?;
        let lines: Vec<String> = content
            .lines()
            .map(|line| line.to_string())
            .collect();
        Ok(lines)
    }


}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_validate_path() {
        let path = "./src/main.rs";
        let result = FileHandling::validate_path(path);
        assert!(result.is_ok());
        
        let path = "./src/nofile_wrong_resutl.rs23434";
        let result = FileHandling::validate_path(path);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_read_file() {
        let path = "./src/main.rs";
        let result = FileHandling::read_file(path);
        assert!(result.is_ok());

        let path = "./src/nofile_wrong_result.rs23434";
        let result = FileHandling::read_file(path);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_read_lines() {
        let path = "./src/main.rs";
        let result = FileHandling::read_lines(path);
        assert!(result.is_ok());

        let path = "./src/nofile_wrong_result.rs23434";
        let result = FileHandling::read_lines(path);
        assert!(!result.is_ok());
    }
}

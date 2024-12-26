use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_files_recursively(directory: &str) -> Vec<PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_files_recursively() {
        let directory = "./src";
        let result = find_files_recursively(directory);
        assert_ne!(result.len(), 0);
        println!("{:#?}", result);

        let directory = "./sjdfids/wer2243/sajdkfvcxcc";
        let result = find_files_recursively(directory);
        assert_eq!(result.len(), 0);
    }
}

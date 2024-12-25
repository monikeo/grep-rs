use crate::model::Config;
use crate::file::FileError;

use regex::Regex;

pub fn search_lines(config: &Config) -> Result<Vec<String>, FileError> {
    let regex = if config.get_ignore_case() {
        Regex::new(&format!("(?i){}", config.get_query())).unwrap()
    } else {
        Regex::new(config.get_query()).unwrap()
    };

    let lines = config.read_lines()?;

    Ok(lines.iter()
        .enumerate()
        .filter(|(_, line)| regex.is_match(line) != config.get_invert_match())
        .map(|(index, line)| {
            if config.get_line_number() {
                format!("{}: {}", index+1, line)
            } else {
                line.to_string()
            }
        }).collect())
}


#[cfg(test)]
mod tests {
    use super::*;

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
}

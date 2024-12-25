use std::process;
pub mod file;
pub mod model;
pub mod search;
pub mod recursive;
use model::Config;

pub fn error_exit(message: &str) {
    println!(" [-] ERROR: {}", message);
    process::exit(0x0100);
}

pub fn search_lines(pattern: &str, config: Config) -> Vec<String> {
    todo!()
}

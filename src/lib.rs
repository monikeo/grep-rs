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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}

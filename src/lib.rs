use std::process;
pub mod file;
pub mod model;

pub fn error_exit(message: &str) {
    println!(" [-] ERROR: {}", message);
    process::exit(0x0100);
}

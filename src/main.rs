mod cli;
mod file;
mod search;
mod recursive;
mod model;
use cli::Args;

use clap::Parser;

fn main() {
    let args = Args::parse_args();
    println!("{:?}", args);
}

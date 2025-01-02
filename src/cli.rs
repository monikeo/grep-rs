use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    version="0.1.0", 
    about="Search for PATTERNS in each FILE", 
    author="Keo Moni",
    long_about = None)]
pub struct Args {
    #[arg(short, long, help = "The pattern to search for")]
    pattern: String,
    #[arg(short = 'f', long = "file", help = "The file to search")]
    file: Option<String>,
    #[arg(short = 'F', long = "files", help = "The files to search")]
    files: Option<Vec<String>>,
    #[arg(short = 'd', long = "directory", help = "The directory to recursively search")]
    directory: Option<String>,
    #[arg(
        short = 'i',
        long = "ignore-case",
        default_value = "false",
        help = "Ignore case distinctions"
    )]
    ignore_case: bool,
    #[arg(
        short = 'R',
        long = "recursive",
        default_value = "false",
        help = "Search directories recursively"
    )]
    recursive: bool,
    #[arg(
        short = 'n',
        long = "line-number",
        default_value = "false",
        help = "Show line numbers with output lines"
    )]
    line_number: bool,
    #[arg(short = 'v', long = "invert-match", default_value = "false", help = "Invert the match")]
    invert_match: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
    pub fn get_pattern(&self) -> &str {
        &self.pattern
    }
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }
    pub fn get_files(&self) -> Option<&Vec<String>> {
        self.files.as_ref()
    }
    pub fn get_directory(&self) -> Option<&String> {
        self.directory.as_ref()
    }
    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }
    pub fn get_recursive(&self) -> bool {
        self.recursive
    }
    pub fn get_line_number(&self) -> bool {
        self.line_number
    }
    pub fn get_invert_match(&self) -> bool {
        self.invert_match
    }
}

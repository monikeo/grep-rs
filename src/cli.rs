use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    version="0.1.0", 
    about="Search for PATTERNS in each FILE", 
    author="Keo Moni",
    long_about = None)]
pub struct Args {
    #[arg(short, long, help="The pattern to search for")]
    pattern: Option<String>,
    #[arg(short, long, help="The files to search")]
    files: Option<String>,
    #[arg(short, 
        long="ignore-case", 
        help="Ignore case distinctions")]
    ignore_case: Option<String>,
    #[arg(short='r',
        long="recursive",
        help="Search directories recursively")]
    recursive: Option<String>,
    #[arg(short='n',
        long="line-number",
        help="Show line numbers with output lines")]
    line_number: Option<String>,
    #[arg(short='v',
        long="invert-match",
        help="Invert the match")]
    inivert_match: Option<String>
}

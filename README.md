# grep-rs
learn to cloning the grep tool in cli


# 1. Define the Scope of the Project
- Basic regex search
- File and directory handling
- Output formatting
- Pattern matching with various options (e.g., case-insensitive, whole words,
  etc.))

# 2. Dependencies (crates)
- clap : For command-line argument parsing.
- regex : For pattern matching (we'll use Rust's regex library)
- walkdir : For recursive directory traversal

# 3. Design the Program's Structure
- main.rs : Entry point to handle argument parsing and setting up execution
- cli.rs : Parse and structure command-line options
- file_search.rs : Handle searching within files
- regex_util.rs : Utility functions for different regex modes (basic, extended,
  etc.)
- output.rs : Handle output formatting (e.g., line numbers, file names, etc.).

# 4. Define the Arguments Structure
- Make pattern and files required: ensure that a pattern is provided, and at least one file should be specified.
- Convert options to booleans: Some flags like ignore_case, recursive, line_number, and invert_match should be bool since they represent the presence or absence of an option
- pattern and files are required arguments, and the boolean flags (ignore_case, recursive, line_number, and invert_match) will default to false if not specified.

# 5.Implement the Search Logic
- Handle Pattern Matching: We will use the regex crate to match the pattern in the files. If the ignore_case flag is set, we'll compile the regex with case insensitivity.
- Read Files: You will need to read files line by line. If recursive is set, you should search through all files in directories recursively.
- Invert Matches: If invert_match is true, we will show lines that do not match the pattern.
- Show Line Numbers: If line_number is true, we will display the line number for each match.

# 6. CLI Options and Functionality
1. Pattern (-p or --pattern)
    - The string or regex to search for in the files.
2. File (-f or --file)
    - Single file to search
3. Files (-F or --files)
    - Multiple files to search
4. Ignore Case (-i or --ignore-case)
    - Perform case-insensitive search
5. Recursive (-R or --recursive)
    - Search directories recursively
6. Line Number (-n or --line-number)
    - Display line numbers in the results
7. Invert Match (-v or --invert-match)
    - Invert the pattern match logic

# 7. CLI Example

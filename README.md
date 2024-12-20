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

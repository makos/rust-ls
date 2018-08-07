use std::env;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

// String literals for command-line switches.
const ALL_FILES_FLAG: &str = "-a";
const HELP_FLAG: &str = "-h";

struct Options {
    all_files: bool,
    help: bool,
    path: String,
}

impl Options {
    pub fn new(args: &Vec<String>) -> Options {
        // If no path or switches specified, app defaults to current directory.
        let mut all_files = false;
        let mut help = false;
        let mut path = String::from("./");

        // [1..] to skip the app name which has index 0.
        for arg in &args[1..] {
            // [..] to pass string slices (&str) instead of String.
            match &arg[..] {
                ALL_FILES_FLAG => all_files = true,
                HELP_FLAG => help = true,
                // All other arguments to be treated as paths.
                target_path @ _ => {
                    path.clear();
                    path = target_path.to_string();
                }
            }
        }

        Options {
            all_files,
            help,
            path,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = Options::new(&args);
    let path = Path::new(&opts.path);
    // Should the output be printed line-by-line (true) or next to each other (false)?
    let mut newline = false;

    // Print help and exit.
    if opts.help {
        print_help(&args[0]);
        return;
    }

    if let Ok(dirs) = path.read_dir() {
        let mut dirs: Vec<Result<DirEntry, io::Error>> = dirs.collect();

        if dirs.len() > 10 {
            newline = true;
        }

        for entry in dirs {
            if let Ok(entry) = entry {
                // Show all files, even hidden ones.
                if opts.all_files {
                    parse_metadata_and_print(newline, &entry);
                // Ignore files starting with a dot.
                } else {
                    if !entry.file_name().into_string().unwrap().starts_with(".") {
                        parse_metadata_and_print(newline, &entry);
                    }
                }
            }
        }
    }
    // Add newline after all output is done.
    println!("");
}

/// Checks if given DirEntry is a directory or a file and passes execution to appropriate printing method.
/// `newline` specifies whether output should use `println!` instead of `print!`.
///
/// # Example
///
/// Consider this directory layout:
/// ````
/// ./ _
///    | - .gitignore
///    | - file.txt
///    | - somedir/
/// ````
///
/// ````
/// let newline = false;
/// let items = std::path::Path::new("./").read_dir().unwrap(); // returns an iterator over DirEntry instances
/// for item in items {
///     if let Ok(item) = item {
///         parse_metadata_and_print(newline, &item);
///     }
/// }
/// ````
/// Output:
/// ````
/// file.txt  somedir/
/// ````
fn parse_metadata_and_print(newline: bool, entry: &DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
            print_dir(newline, &entry.file_name().into_string().unwrap());
        } else {
            print_file(newline, &entry.file_name().into_string().unwrap());
        }
    }
}

/// Print out directory (filepath with `/` at the end).
///
/// # Example
///
/// ````
/// let newline = false;
/// let dir = "mydir";
/// print_dir(newline, dir);
/// ````
/// Output:
/// ````
/// mydir/
/// ````
fn print_dir(newline: bool, dir: &str) {
    if newline {
        println!("{}/", dir);
    } else {
        print!("{}/  ", dir);
    }
}

/// Print out directory (filepath with `/` at the end).
///
/// # Example
///
/// ````
/// let newline = false;
/// let file = "myfile";
/// print_file(newline, file);
/// ````
/// Output:
/// ````
/// myfile
/// ````
fn print_file(newline: bool, file: &str) {
    if newline {
        println!("{}", file);
    } else {
        print!("{}  ", file);
    }
}

/// Print out help text.
fn print_help(name: &String) {
    print!(
        "\
{n} (Rust list) is an ls-like utility to list directory contents.

Usage: {n} [OPTIONS]
Options:
-h  -- this help
-a  -- show all files
",
        n = name
    );
}

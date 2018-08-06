use std::env;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

const ALL_FILES_FLAG: &str = "-a";
const HELP_FLAG: &str = "-h";

struct Options {
    all_files: bool,
    help: bool,
    path: String,
}

impl Options {
    pub fn new(args: &Vec<String>) -> Options {
        let mut all_files = false;
        let mut help = false;
        let mut path = String::from("./");

        for arg in &args[1..] {
            match &arg[..] {
                ALL_FILES_FLAG => all_files = true,
                HELP_FLAG => help = true,
                a @ _ => {
                    path.clear();
                    path = a.to_string();
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

    let mut newline = false;

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
                // Show all files, even hidden ones
                if opts.all_files {
                    parse_metadata_and_print(newline, &entry);
                // Ignore files starting with a dot
                } else {
                    if !entry.file_name().into_string().unwrap().starts_with(".") {
                        parse_metadata_and_print(newline, &entry);
                    }
                }
            }
        }
    }
    println!("");
}

fn parse_metadata_and_print(newline: bool, entry: &DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
            print_dir(newline, &entry.file_name().into_string().unwrap());
        } else {
            print_file(newline, &entry.file_name().into_string().unwrap());
        }
    }
}

fn print_dir(newline: bool, dir: &str) {
    if newline {
        println!("{}/", dir);
    } else {
        print!("{}/  ", dir);
    }
}

fn print_file(newline: bool, file: &str) {
    if newline {
        println!("{}", file);
    } else {
        print!("{}  ", file);
    }
}

fn print_help(name: &String) {
    print!(
        "\
{n} (Rust list) is an ls-like utility to list directory contents.

Usage: {n} [OPTIONS]
Options:
-h  -- this help
-a  -- show all files\n",
        n = name
    );
}

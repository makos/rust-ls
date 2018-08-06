use std::env;
use std::path::Path;
use std::io;
use std::fs::DirEntry;

const ALL_FILES:    &str    = "-a";
const HELP:         &str    = "-h";

struct Options {
    all_files: bool,
    help: bool,
    path: Path
}

impl Options {
    pub fn new(args: &Vec<String>) -> Self {
        let mut all_files = false;
        let mut help = false;
        let mut path = Path::new("./");

        for arg in args {
            match arg {
                ALL_FILES = { all_files = true },
                HELP = { help = true },
                a @ _ = { path = Path::new(a)},
            }
        }

        Self {
            all_files,
            help,
            path
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut newline = false;
    let path = Path::new(&args[1]);

    if let Ok(dirs) = path.read_dir() {
        let dirs: Vec<Result<DirEntry, io::Error>> = dirs.collect();

        if dirs.len() > 10 {
            newline = true;
        }

        for entry in dirs {
            if let Ok(entry) = entry {
                // Don't display entries starting with dot
                if !entry.file_name().into_string().unwrap().starts_with(".") {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            print_dir(newline, &entry.file_name().into_string().unwrap());
                        } else {
                            print_file(newline, &entry.file_name().into_string().unwrap());
                        }
                    }
                }
            }
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

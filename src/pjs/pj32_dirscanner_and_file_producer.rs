use std::fs::{OpenOptions, read_dir};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process::exit;

pub fn dir() {
    let dir = "/mnt/workspace/Projects/ms-projects/src/pjs";
    let path = Path::new(&dir);
    if path.exists() && path.is_dir() {
        println!("{}", path.to_str().unwrap());

        //scan_dir(&path, 0)
        scan_dir_without(
            path,
            0,
            Path::new("/mnt/workspace/Projects/ms-projects/target"),
        )
    } else {
        println!("{} not a directory in file system", dir)
    }
}
fn scan_dir(path: &Path, depth: usize) {
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let pathname = entry.file_name().into_string().unwrap();
                let filetype = entry.file_type().unwrap();
                // 1. Create the spacing for the depth
                let indent = "│   ".repeat(depth);
                // 2. Use a clean branch pointer instead of just a hyphen
                let branch = "└── ";

                if filetype.is_dir() {
                    println!("{}{}📁 {}", indent, branch, pathname);
                    scan_dir(&entry.path(), depth + 1);
                } else if filetype.is_file() {
                    println!("{}📄 {}", indent, pathname);
                }
            }
        }
    }
}
// add exclude
// add level of search
// build large file contains all the content from sub-dirs
fn scan_dir_without(path: &Path, depth: usize, exclude: &Path) {
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let pathname = entry.file_name().into_string().unwrap();
                let filetype = entry.file_type().unwrap();

                // 1. Create the spacing for the depth
                let indent = "│   ".repeat(depth);
                // 2. Use a clean branch pointer instead of just a hyphen
                let branch = "└── ";

                if filetype.is_dir() {
                    println!("{}{}📁 {}", indent, branch, pathname);
                    if entry.path() != exclude {
                        scan_dir_without(&entry.path(), depth + 1, exclude);
                    }
                } else if filetype.is_file() {
                    println!("{}📄 {}", indent, pathname);
                    file_builder(&entry.path(), Path::new("/mnt/workspace/Projects/ms-projects/assets/entire_dir_file.txt"))
                }
            }
        }
    }
}
fn file_builder(path: &Path, output: &Path) {
    let file = if let Ok(file) = OpenOptions::new()
        .read(true)
        .open(path)
    {
        file
    } else {
        eprintln!("could not open the file {}", path.to_str().unwrap());
        exit(0)
    };
    let output_file = if let Ok(file) = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(output)
    {
        file
    } else {
        eprintln!("could not open the file {}", output.to_str().unwrap());
        exit(0)
    };
    let mut reader = BufReader::new(file);
    let mut writer = BufWriter::new(output_file);

    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    write!(writer,"{}",string).unwrap();
    writer.flush().unwrap()
}

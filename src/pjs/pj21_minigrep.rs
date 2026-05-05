// search for a pattern in a specific place
// -- searchstring

// this is my own version

use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader};

pub fn grep() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        eprintln!("the command should be like this \"cargo run -- searchstring path_to_file\"");
        return;
    }
    let searchstring = if let Some(arg) = args.get(1) {
        arg.trim()
    } else {
        return;
    };
    let filepath = if let Some(arg) = args.get(2) {
        arg.trim()
    } else {
        return;
    };
    if !std::fs::exists(filepath).unwrap_or_else(|e| {
        eprintln!("path access denied,{}", e);
        true
    }) {
        return;
    }
    let mut found = false;
    let file = OpenOptions::new().read(true).open(filepath).unwrap();
    let reader = BufReader::new(file);
    for (number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("the file found empty, {}", e);
                break;
            }
        };
        let number = number + 1;
        if line.contains(searchstring) {
            println!("[line #{}]: {}",number,line);
            found = true;
        }
    }
    if !found {
        println!("we didnt find any match at the given path")
    }

}



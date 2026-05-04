// reads a file ,.tomm , .init
// generates key-value pairs and output them into the terminal
// using serde

// problems
// how the toml file looks like , initially it have name =  value , so it can be represented by key-value
// but some file contains [....]  is this for human ux, or it has internal mechanism ?
// hos to avoid comments
// the left hand side of toml always string, but the right differs, how i can handle this ?using generics ? parse directly to string ?

// final plan
// the user inputs the file path
// the function parses the file inot key valu pairs
// ouputs them after finishes parsing
// any line with error parsing, output it

// static path

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write, stdin, stdout};

pub fn toml_parser() -> io::Result<()> {
    println!("welcome to toml parser");

    loop {
        print!("enter the file path or \"exit\" to proceed: ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let path = input.trim();

        if path.to_lowercase() == "exit" {
            break;
        }

        if std::fs::metadata(path).is_ok() {
            println!("Opening: {}", path);
            parse(path);
        } else {
            println!("File \"{}\" does not exist or is inaccessible.", path);
        }
    }
    Ok(())
}
fn parse(path: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        let nbr = index + 1;

        match line_result {
            Ok(line) => {
                if check_line(&line) {
                    let trimmed = line.trim();
                    let vector: Vec<&str> = trimmed.split_whitespace().collect();

                    if vector.len() >= 3 {
                        map.insert(vector[0].to_lowercase(), vector[2].to_string());
                        println!("{}: {}", nbr, trimmed);
                    }
                } else {
                    println!("Line {} invalid format: {}", nbr, line);
                }
            }
            Err(e) => println!("Error reading line {}: {}", nbr, e),
        }
    }
    map
}

fn check_line(line: &str) -> bool {
    let line = line.trim().to_lowercase();
    let mut vector: Vec<&str> = line.split_whitespace().collect();
    match vector.get(1) {
        Some(value) => {
            if !value.eq(&"=") {
                false
            } else {
                true
            }
        }
        None => false,
    }
}

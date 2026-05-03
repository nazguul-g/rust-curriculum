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
// any line with error parsing, outpout it

// static path

use std::io;
use std::io::{stdin, stdout, Write};

pub fn toml_parser() -> io::Result<()> {
    println!("welcome to toml parser");
    loop {
        print!("enter the file path or \"exit\" to proceed: ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "exit" => break,
            path if std::fs::exists(input)? => todo!(),
            _ => {
                println!("please enter a valid path ");
                continue
            }

        }

    }
    Ok(())
}
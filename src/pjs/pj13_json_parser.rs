use std::env;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn json_parser() {
    let args: Vec<String> = env::args().collect();

    // in many os the first argument is the path of the executable
    // the cargo run is parameters passed to cargo not the compiler
    // the final command is like this :
    // executable_path path_to_file
    if args.len() != 2 {
        eprintln!("usage: cargo run <path to json file>");
        return;
    }
    println!("{:?}", args);
    let path = &args[1];

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("error open the specified path");
            return;
        }
    };

    let reader = BufReader::new(file);
    let content:Value = serde_json::from_reader(reader).unwrap();

    println!("{:?}",content)
}

// the idiomatic way to do this is to represent the argument in struct

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};
use std::process;

struct Config {
    pattern: String,
    path: String,
    case_sensitive: bool,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        // in the result error type rust automatically infers 'static lifetime to it
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let pattern = if let Some(string) = args.get(1) {
            string.clone()
        } else {
            return Err("error getting the pattern");
        };
        let path = if let Some(path) = args.get(2) {
            path.clone()
        } else {
            return Err("error getting the pattern");
        };
        let case_sensitive = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            pattern,
            path,
            case_sensitive,
        })
    }
}
// we need a runner function to orchestrate

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(config.path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let result = if config.case_sensitive {
        search(&config.pattern, &content)
    } else {
        search_without_case(&config.pattern, &content)
    };
    for line in result {
        println!("{}", line)
    }
    Ok(())
}

fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut vector = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            vector.push(line)
        }
    }
    vector
}

fn search_without_case<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut vector = Vec::new();
    let pattern = pattern.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&pattern) {
            vector.push(line)
        }
    }
    vector
}

pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("error parsing the arguments {}", e);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("application error {}",e);
        process::exit(1);
    }
}

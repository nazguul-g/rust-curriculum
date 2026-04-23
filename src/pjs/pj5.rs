use std::fs::File;
use std::{env, io};
use std::io::{Error, ErrorKind, Read};


pub fn word_counter () -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("the usage is : cargo run <file_path>");
        return Err(Error::new(ErrorKind::InvalidInput,"invalid input"))
    }
    let filepath = &args[1];
    println!("{:?}",args);
    let mut file = File::open(filepath)?;
    let mut f = String::new();
    file.read_to_string(&mut f )?;
    let words = f.split_whitespace().count();
    let lines = f.lines().count();
    let chars = f.chars().filter(|c| !c.is_whitespace()).count();
    println!("words = {}",words);
    println!("lines = {}",lines);
    println!("characters = {}",chars);
    Ok(())
}
#[test]
fn test () {
    word_counter().unwrap();
}
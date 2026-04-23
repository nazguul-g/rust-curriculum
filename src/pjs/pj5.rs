use std::fs::File;
use std::io;
use std::io::Read;

pub fn word_counter () -> io::Result<()> {
    let mut file = File::open("words_for_pj5.txt")?;
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
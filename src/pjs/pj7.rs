use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind};

pub fn palindrome_checker() -> io::Result<()> {
    // input a file contains words
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "invalid input, the input should be like this cargo run <file_path>",
        ));
    }
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(&file);
    let output = File::create("assets/palindromeoutput.txt")?;
    let mut writer = BufWriter::new(output);
    for line in reader.lines() {
        let line = &line?;
        if is_pal(line) {
            writeln!(writer, "{}", line)?;
        }
    }
    writer.flush()?;
    Ok(())
}
pub fn is_pal(word: &String) -> bool {
    word.to_lowercase().eq(&word
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .rev()
        .collect::<String>())
}

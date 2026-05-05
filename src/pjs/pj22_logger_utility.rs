use std::fs::OpenOptions;
use std::io::{stdin, stdout, BufWriter, Write};
use chrono::Local;

pub fn logger_util ()  {
    loop {
        println!("choose option to log:\n\t1- INFO\n\t2- ERROR\n\t3- WARNING\n\t4- EXIT ");
        print!("option: ");
        let input = input("choice: ");
        match input.as_str() {
            "1" => logger("INFO"),
            "2" => logger("ERROR"),
            "3" => logger("WARNING"),
            "4" => break,
            _ => continue,
        }
    }
}
fn logger (level :&str) {
    let file = OpenOptions::new().create(true).write(true).append(true).open("assets/log.txt").unwrap();
    let mut writer= BufWriter::new(file);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let message = input(&format!("enter {} message:", level));

    writeln!(writer,"[{}]-{}:{}",timestamp,level,message).expect("error writing file");
}
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
// objective:
// like the timer in the mobile , you enter the time and after that time passed reminds you with some kind of action

use std::io::{Write, stdin, stdout};

pub fn timer_tool() {
    println!("{:?}", get_time_input())
}
fn get_time_input() -> Option<(u64, u64, u64)> {
    println!("welcome to timer ");
    print!("enter the time (e,g,. HH:MM:SS): ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("ERROR: failed to read input text");
    // to lowercase created new temp string , in the end the string is dropped , the vectore became pointing to nothing (dangling ptr)
    let vector: Vec<&str> = input.trim().split(':').collect();
    if &vector.len() != &3 {
        return None;
    }

    let hours: u64 = vector[0].parse().ok()?;
    let minutes: u64 = vector[1].parse().ok()?;
    let seconds: u64 = vector[2].parse().ok()?;
    Some((hours, minutes, seconds))
}
#[test]
fn test() {
    timer_tool()
}

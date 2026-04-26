// objective:
// like the timer in the mobile , you enter the time and after that time passed reminds you with some kind of action

use std::io::{Write, stdin, stdout};
use std::thread;
use std::time::Duration;

pub fn timer_tool() {

    timer(&get_time_input())
}
fn get_time_input() -> Vec<u64> {
    println!("welcome to timer ");
    let mut vector = Vec::with_capacity(3);
    loop {
        print!("enter the time (e,g,. HH:MM:SS): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("ERROR: failed to read input text");
        // to lowercase created new temp string , in the end the string is dropped , the vector became pointing to nothing (dangling ptr)
        vector = input.trim().split(':').filter_map(|str| str.parse::<u64>().ok()).collect();
        if vector.len() != 3 {
            println!("enter a valid time");
            continue;
        }
        break;
    }
    vector
}
fn timer(time: &Vec<u64>) {
    let mut total_time = time[0] * 3600 + time[1] * 60 + time[2];
    while total_time > 0 {
        let hours = total_time / 3600;
        let minutes = (total_time/3600) % 60 ;
        let seconds = total_time % 60;

        // \r for making the cursor of writing pointing to same place , so we overwrite previous time
        print!("\r{:02}:{:02}:{:02}",hours,minutes,seconds);
        let _ =  stdout().flush();
        thread::sleep(Duration::from_secs(1));
        total_time-=1;
    }
}
#[test]
fn test() {
    timer_tool()
}

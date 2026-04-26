use std::io::{Write, stdin, stdout};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn timer_tool() {
    let mut vector: Vec<u64> = Vec::with_capacity(3);
    println!("welcome to basic timer");
    loop {
        print!("enter time (e.g. hh:mm:ss): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("failed to read the input ");
        vector = input
            .trim()
            .split(":")
            .filter_map(|time| match time.parse::<u64>() {
                Ok(val) => Some(val),
                Err(_) => {
                    println!("'{}' is not a valid number, skipping...", time);
                    None
                }
            })
            .collect();

        if vector.len() == 3 { break } else { continue }
    }
    let mut total_seconds = vector[0] * 3600 + vector[1] * 60 + vector[2];
    println!("press 'p' for pause or press 'r' for resume");
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim().to_lowercase().as_str() {
                "p" => {
                    println!("paused");
                    tx.send(true).unwrap()
                }
                "r" => {
                    println!("resumed");
                    tx.send(false).unwrap()
                }
                _ => {
                    println!("invalid key");
                    continue;
                }
            }
        }
    });
    let mut paused = false; // Start NOT paused (timer running)

    // main thread
    while total_seconds > 0 {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        print!(
            "\rremaining time is {:02} hours , {:02} minutes and {:02} seconds",
            hours, minutes, seconds
        );
        let _ = stdout().flush();

        if let Ok(new_paused_state) = rx.try_recv() {
            paused = new_paused_state;
        }

        if !paused {
            thread::sleep(Duration::from_secs(1));
            total_seconds -= 1;
        }
    }
    println!("\n⏰ Timer finished!");
}

use std::thread;
use std::time::{Duration, Instant};

struct Task {
    name: String,
    delay: u64,
}

pub fn simulator() {
    let task1 = Task {
        name: "Cleanup files".into(),
        delay: 3,
    };
    let task2 = Task {
        name: "Open we browser".into(),
        delay: 5,
    };
    let task3 = Task {
        name: "Close other windows".into(),
        delay: 1,
    };

    let mut tasks = vec![task3, task2, task1];
    tasks.sort_by_key(|t| t.delay);
    let start = Instant::now();
    for task in tasks {
        let wait = Duration::from_secs(task.delay);
        println!("waiting {}s for starting \"{}\"", task.delay, task.name);
        thread::sleep(wait);
        let elapsed = Instant::now().duration_since(start).as_secs();
        println!("✅ [{}s] Task \"{}\" completed", elapsed, task.name);
    }
    println!("all tasks completed");
}

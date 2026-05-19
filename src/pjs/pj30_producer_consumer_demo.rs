// classic problem where producers producer values into a buffer and consumer consumes it
// the problem is control the producers and consumers so we want resulted by buffer overflow or empty buffer

use std::thread;
use std::time::Duration;

pub fn producer_consumer() {
    let buffer_size = 3;
    let (tx, rx) = std::sync::mpsc::sync_channel(buffer_size);

    // producer thread
    let producer = thread::spawn(move || {
        println!("producing values");
        
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(300))
        }
    });

    // consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            println!("received value {task}");
            thread::sleep(Duration::from_millis(300))
        }
    });

    consumer.join().unwrap();
    producer.join().unwrap();
    thread::sleep(Duration::from_secs(5));
    println!("all tasks finished")
}

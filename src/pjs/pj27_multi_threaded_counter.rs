use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
//A CLI multi-threaded counter that spawns multiple threads, each incrementing a shared value.

pub fn multithreadedcounter() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for i in 0..5 {
        let cloned_counter = Arc::clone(&counter);
        for _ in 0..1000 {}
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                cloned_counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
        println!("thread {} finished.", i)
    }

    for handle in handles {
        handle.join().expect("thread panicked")
    }
    println!("{}", counter.load(Ordering::SeqCst))
}


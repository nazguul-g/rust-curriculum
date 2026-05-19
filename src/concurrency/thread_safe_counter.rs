// multiple threads try to increment a shared counter
// Build a CLI tool that spawns N threads, each incrementing a shared counter M times, and prints the final correct value.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::thread::JoinHandle;

pub fn shared_counter_with_mutex () {

    // this is a bad type , race condition
    // let mut counter = 0_usize;
    let counter= Arc::new(Mutex::new(0));

    let threads_nbr = 8;
    let mut handles: Vec<JoinHandle<()>> = (0..threads_nbr).map(|thread|{
        let mut counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter.lock().unwrap();
                *guard+=1;


            }
            println!("thread {thread} finished its job")
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap()
    }
    let result = counter.lock().unwrap();
    println!("counter= {result}")

}

pub fn shared_counter_with_atomictype () {

    // this is a bad type , race condition
    // let mut counter = 0_usize;
    let counter= Arc::new(AtomicI32::new(0));

    let threads_nbr = 8;
    let mut handles: Vec<JoinHandle<()>> = (0..threads_nbr).map(|thread|{
        let mut counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("thread {thread} finished its job")
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap()
    }
    let result = counter.load(Ordering::SeqCst);
    println!("counter= {result}")

}
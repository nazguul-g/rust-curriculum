// multiple threads try to increment a shared counter
// Build a CLI tool that spawns N threads, each incrementing a shared counter M times, and prints the final correct value.

use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn shared_counter () {

    // this is a bad type , race condition
    // let mut counter = 0_usize;
    let counter= Arc::new(Mutex::new(0));

    let threads_nbr = 8;
    let mut handles: Vec<JoinHandle<()>> = (0..threads_nbr).map(|thread|{
        let mut counter = Arc::clone(&counter);
        thread::spawn(move || {
            for i in 0..1000 {
                let mut guard = counter.lock().unwrap();
                *guard+=1;


            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap()
    }


}
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
//A CLI multi-threaded counter that spawns multiple threads, each incrementing a shared value.

pub fn multithreadedcounter() {
    let counter = Arc::new(AtomicUsize::new(0));
    // why not  Arc::new(0); tho ?
    // the usual data type have immutable or mutable states , which is any other part of code can use it , this destroys the concept of shared counter
    // the only solution is create new abstracted data type for this purpose

    let mut handles = vec![];

    for i in 0..5 {
        // this is bad act ; we dont know if its a shallow copy or deep copy
        // let cloned_counter = counter.clone();

        // here we're just incrementing the reference counter
        let cloned_counter = Arc::clone(&counter);

        //we want each thread increment the counter by 1000
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                cloned_counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        println!("thread {} finished", i);
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }
    println!("{}", counter.load(Ordering::SeqCst))
}

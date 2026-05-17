use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;



type Job = Box<dyn Send + FnOnce() + 'static>;
// we don't know what the task is
// Send is a trait bound  makes the type safer send between threads
// dyn for letting runtime decide size
// FnOnce() makes task used only one time in the thread

// dyn means each type implements that trait
// the dyn applied for both FnOnce() behavioral trait and Send marker trait
// Send marker trait is a trait that makes a type safe to share between threads
// With + Send
// let task2: Box<dyn FnOnce() + Send> = Box::new(|| {});
// task2 explicitly includes Send in the trait object
// the Box have points to a type its bound is Send

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = {
                    if let Ok(guard) = receiver.lock() {
                        guard.recv()
                    } else {
                        eprintln!("error unlocking mutex");
                        break;
                    }
                };
                match message {
                    Ok(job) => {
                        println!("worker {id} received a job");
                        job()
                    }
                    Err(_) => {
                        eprintln!("no message received , shutting down thread {id}");
                        break;
                    }
                }
            }
        });
        Worker {
            handle: Some(thread),
            id,
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let mut workers = Vec::new();

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker)
        }

        ThreadPool {
            workers,
            sender: Some(tx),
        }
    }
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = if let Some(sender) = &self.sender {
            sender
        } else {
            eprintln!("found no sender");
            return;
        };

        match sender.send(Box::new(f)) {
            Ok(_) => {
                println!("sent job to worker ");
            }
            Err(_) => {
                eprintln!("failed to send job")
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

pub fn thread_pool() {
    let thread_pool = ThreadPool::new(5);
    thread_pool.execute(Box::new(|| {
        for x in 0..5 {
            println!("{x}")
        }
    }));
    thread::sleep(Duration::from_secs(1))
}

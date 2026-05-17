use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{JoinHandle, spawn};

// we don't know what the task is
type Task = Box<dyn FnOnce() + Send + 'static>; // safe to transfer across threads
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

struct ThreadPool {
    // each worker is responsible for a task
    workers: Vec<Worker>,

    // we send the task into threads
    sender: Option<Sender<Task>>,
}
struct Worker {
    handle: JoinHandle<()>,
    id: usize,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
        let thread = spawn(move || {
            loop {
                let task = { receiver.lock().unwrap().recv().unwrap() };
                println!(" worker {} got a job", id);
                task();
            }
        });
        Worker { handle: thread, id }
    }
}
impl ThreadPool {
    // how many worker do we need as input param
    fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        // wrap up the receiver to be shared across threads
        let receiver = Arc::new(Mutex::new(rx));
        let workers = (0..size)
            .map(|id| {
                let worker = Worker::new(id, Arc::clone(&receiver));
                worker
            })
            .collect();
        ThreadPool {
            workers,
            sender: Some(tx),
        }
        // returns x size of workers with the sender channel
    }
    fn execute(&self, task: Task) {
        self.sender.as_ref().unwrap().send(task).unwrap()
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender = self.sender.take();
    }

}

pub fn thread_pool() {
    let thread_pool = ThreadPool::new(5);
    thread_pool.execute(Box::new(|| {
        for x in 0..5 {
            println!("{x}")
        }
    }))
}

// a simple webcrawler
// returns request status of a vector of URLs

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use reqwest::blocking::get;

pub fn crawler() {

    let urls = vec![
      "https://crates.io/",
      "https://rust.learningz.xyz/books/100-rust-projects/simple_web_crawler",
      "https://www.npr.org/",
      "https://web.archive.org/",
      "https://monkeytype.com/",
      "https://www.youtube.com/",
    ];

    let (tx, rx) = channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = Vec::new();
    let pool_size = 8_usize ;
    for i in 0..pool_size {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let url = match rx.lock().unwrap().recv() {
                    Ok(url) => url,
                    Err(_) => {
                        break
                    }

                };
                match fetch_url(&url) {
                    Ok(status) => println!("worker # {} , ✅ {} => {}",i, url, status),
                    Err(e) => println!("worker # {} , ❌ {} => {}",i, url, e),
                }
            }

        });
        handles.push(handle)
    }
    for url in urls  {
        tx.send(url.to_string()).unwrap()
    }
    drop(tx);
    for handle in handles {

        handle.join().unwrap()
    }
    println!("finished")

}
fn fetch_url(url:&str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    // blocking get which it means synchronous task
    Ok(format!("{} returned {}", url, response.status()))
}

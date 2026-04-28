// the goal is to perform basic CRUB operation of a data
// I extended  it to be working with files (serde)

// two entry points into file
// read and write only , no update directly or delete directly

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
// define the structure of contact

#[derive(Serialize, Deserialize)]
struct Contact {
    id: usize,
    full_name: String,
    number: u32,
    address: String,
    email: String,
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Contact {
    // create
    fn new(full_name: String, number: u32, address: String, email: String) -> Self {
        Contact {
            id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            full_name,
            number,
            address,
            email,
        }
    }
    // update
}

pub fn manager() {}

// read file and return the vector of contacts
fn read_file(path: &str) -> Vec<Contact> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .expect("ERROR: failed to read the file path ");

    let mut reader = BufReader::new(file);
    let contacts: Vec<Contact> = serde_json::from_reader(&mut reader).unwrap_or_else(|_| {
        println!("the file found empty");
        Vec::new()
    });
    contacts
}

// provide vector and write it to file
fn write_file(contacts: &Vec<Contact>, path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("ERROR: error read the file");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &contacts)
        .unwrap_or_else(|e| println!("error serializing the given data , {}", e))
}

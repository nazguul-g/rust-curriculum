// the goal is to perform basic CRUB operation of a data
// I extended  it to be working with files (serde)

// two entry points into file
// read and write only , no update directly or delete directly
// [x] TODO: the id functionality isn't working as it suppose to be

use std::env::var;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write, stdin, stdout};
use std::process::Command;
// define the structure of contact

#[derive(Serialize, Deserialize)]
struct Contact {
    id: u64,
    full_name: String,
    number: String,
    address: String,
    email: String,
}
// i guess no need for this right now
enum ContactTypes {
    FullName(String),
    Number(String),
    Address(String),
    Email(String),
}
impl Contact {
    // create
    fn new(id: u64, full_name: String, number: String, address: String, email: String) -> Self {
        Contact {
            id,
            full_name,
            number,
            address,
            email,
        }
    }
    // good print
    fn print_contact(&self) {
        println!("id: {}", self.id);
        println!("\tfull name: {}", self.full_name);
        println!("\tnumber: {}", self.number);
        println!("\taddress: {}", self.address);
        println!("\temail: {}", self.email);
    }
    // update
    fn update(&mut self, field: ContactTypes) {
        match field {
            ContactTypes::FullName(val) => self.full_name = val,
            ContactTypes::Address(val) => self.address = val,
            ContactTypes::Email(val) => self.email = val,
            ContactTypes::Number(val) => self.number = val,
        }
    }
}


pub fn manager() {
    let mut id = 1;
    println!("welcome to contact manager");
    let path = "assets/contacts.json";
    loop {
        println!("choose an option ");
        println!(
            "\n\t1-create contact\n\t2-Show contacts\n\t3-Update Contact\n\t4-Remove Contact\n\t5-Quit"
        );
        print!("you're option: ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("ERROR reading the input ");
        // we can use read and right on each arm cuz it run per sequence
        match input.trim() {
            "1" => {
                clear_linux_terminal();
                print!("full name: ");
                let _ = stdout().flush();
                let mut full_name = String::new();
                stdin()
                    .read_line(&mut full_name)
                    .expect("ERROR reading the input ");
                print!("address : ");
                let _ = stdout().flush();
                let mut address = String::new();
                stdin()
                    .read_line(&mut address)
                    .expect("ERROR reading the input ");
                print!("email: ");
                let _ = stdout().flush();
                let mut email = String::new();
                stdin()
                    .read_line(&mut email)
                    .expect("ERROR renunmberading the input ");
                print!("number (e,g,. 0555555555): ");
                let _ = stdout().flush();
                let mut number = String::new();
                stdin()
                    .read_line(&mut number)
                    .expect("ERROR reading the input ");
                let contact = Contact::new(
                    id,
                    full_name.trim().to_string(),
                    number,
                    address.trim().to_string(),
                    email.trim().to_string(),
                );

                let mut contacts = read_file(&path);
                contacts.push(contact);
                write_file(&contacts, path);
                id += 1;
            }
            "2" => print(&read_file(&path)),
            "3" => {



            },
            "4" => todo!(),
            "5" => break,

            _ => {
                println!("please provide a valid option from above");
                continue;
            }
        }
    }
}

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
        .unwrap_or_else(|e| println!("error serializing the given data , {}", e));
    println!("successfully written the file ");
}
fn print(contacts: &Vec<Contact>) {
    contacts.iter().for_each(|contact| contact.print_contact())
}

fn clear_linux_terminal() {
    if cfg!(target_os = "windows") {
        return;
    } else {
        let _ = Command::new("clear").status();
    }
}

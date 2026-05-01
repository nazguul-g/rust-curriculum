
use serde::{Deserialize, Serialize};
use std::fs::{ OpenOptions, remove_file};
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ChatEntry {
    pub timestamp: String,
    pub user: String,
    pub message: String,
}
#[derive(Serialize, Deserialize)]
pub struct ChatHistory {
    pub entries: Vec<ChatEntry>,
}

impl ChatHistory {
    pub fn new() -> Self {
        ChatHistory {
            entries: Vec::new(),
        }
    }
    pub fn load(&mut self, path: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;
        let reader = BufReader::new(file);
        *self = serde_json::from_reader(reader).unwrap_or_else(|_| ChatHistory::new());
        Ok(())
    }
    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }
    pub fn clear(&mut self, path: &str) -> io::Result<()> {
        self.entries.clear();
        // let file = OpenOptions::new().write(true).truncate(true).open(path);
        if Path::new(path).exists() {
            remove_file(path)?;
        }
        Ok(())
    }
    pub fn print(&self) {
        self.entries.iter().for_each(|entry| {
            println!(
                "[{}] -> {} : {}",
                entry.timestamp, entry.user, entry.message
            )
        });
    }
    pub fn add_entry(&mut self, timestamp: &str, user: &str, message: &str) {
        self.entries.push(ChatEntry {
            timestamp: timestamp.to_string(),
            user: user.to_string(),
            message: message.to_string(),
        })
    }
}

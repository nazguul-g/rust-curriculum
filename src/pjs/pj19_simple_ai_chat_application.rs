use crate::pjs::bot::history::ChatHistory;
use crate::pjs::bot::responses::RESPONSES;
use chrono::Local;
use std::collections::HashMap;
use std::io;
use std::io::{Write, stdin, stdout};
use std::process::Command;
static PATH: &str = "assets/chatbot_history.json";

struct Chatbot {
    history: ChatHistory,
    responses: HashMap<String, String>,
}
impl Chatbot {
    fn new() -> Self {
        Chatbot {
            history: ChatHistory::new(),
            responses: RESPONSES.clone(),
        }
    }
    fn process_message(&mut self, message: &str) -> String {
        // todo : add simple fuzzy entries
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let user = "Bot";
        if self.responses.contains_key(message) {
            let response = self.responses.get(message).unwrap().to_string();
            self.history.add_entry(&timestamp, "You", message);
            self.history.add_entry(&timestamp, user, &response);
            self.history.save(PATH).unwrap();
            response
        } else {
            self.history.add_entry(&timestamp, "You", message);

            self.history
                .add_entry(&timestamp, user, "I'm not sure how to respond to that.");
            self.history.save(PATH).unwrap();
            "I'm not sure how to respond to that.".to_string()
        }
    }
}
pub fn chat_app() -> io::Result<()> {
    println!("-----welcome to simple chat app -----");
    let mut chatbot = Chatbot::new();
    loop {
        print!("You: ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim();
        match input.to_lowercase().as_str() {
            // TODO: add helper entry
            "clear" => {
                Command::new("clear").status()?;
                chatbot.history.clear(PATH)?;
                println!("history has been cleared");
                continue;
            }
            "quit" => {
                chatbot.history.save(PATH)?;
                break;
            }
            "history" => {
                //TODO: this isn't functional when the bot isn't initialized
                chatbot.history.print();
                continue;
            }
            _ => {
                println!("Bot: {}", chatbot.process_message(&input));
                continue;
            }
        }
    }
    Ok(())
}

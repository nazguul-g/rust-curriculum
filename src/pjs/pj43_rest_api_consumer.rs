use std::error::Error;
use std::io::stdin;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]

struct Joke {
    r#type: String,
    setup: String,
    punchline: String,
    id: u64,
}
pub fn random_joke() ->Result<(), Box<dyn Error>>{
    loop {
        get_input();
        let url = "https://official-joke-api.appspot.com/random_joke";
        let response:Joke = get(url)?.json()?;
        println!("{}",response.setup.bright_green());
        thread::sleep(Duration::from_secs(1));
        println!("{}",response.punchline.bright_green())
    }
}
fn get_input(){
    println!("\rpress enter to fetch joke");
    let mut  dummy = String::new();
    stdin().read_line(&mut dummy).unwrap();
}
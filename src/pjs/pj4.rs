use std::io::{self, Write, stdin, stdout};
use rand::{ RngExt};
#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct Level {
    max_number: u32,
    max_tries: u32,
}

impl Difficulty {
    fn get_settings(&self) -> Level {
        match self {
            Difficulty::Easy => Level { max_tries: 5, max_number: 20 },
            Difficulty::Medium => Level { max_tries: 10, max_number: 50 },
            Difficulty::Hard => Level { max_tries: 20, max_number: 100 },
        }
    }
}

pub fn guess() -> io::Result<()> {
    let mut rng = rand::rng();

    println!("Welcome to our guessing game :)");
    println!("Please choose a difficulty:\n\tA)- Easy\n\tB)- Medium\n\tC)- Hard");

    let difficulty = loop {
        print!("Input: ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "a" => break Difficulty::Easy,
            "b" => break Difficulty::Medium,
            "c" => break Difficulty::Hard,
            _ => println!("Invalid selection, please type A, B, or C."),
        }
    };

    let settings = difficulty.get_settings();
    // Inclusive range so the player can actually guess the max number
    let target = rng.random_range(1..=settings.max_number);

    println!("I'm thinking of a number between 1 and {}.", settings.max_number);

    for attempt in 1..=settings.max_tries {
        print!("Attempt [{}/{}]: Guess a number: ", attempt, settings.max_tries);
        stdout().flush()?;

        let mut entry = String::new();
        stdin().read_line(&mut entry)?;

        // Graceful error handling instead of .expect()
        let guess: u32 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };

        if guess == target {
            println!("Congrats! You guessed it right. The number was {}.", target);
            return Ok(()); // Exit early on win
        } else if guess < target {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }

    println!("That's unfortunate, you lost. The number was {}.", target);
    Ok(())
}
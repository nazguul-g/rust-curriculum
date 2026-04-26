// play rock paper scissors against the computer

// the computer picks random
// and u decide what to choose

use std::cmp::PartialEq;
use rand::RngExt;
use std::io::{Write, stdin, stdout};

#[derive(PartialEq)]
enum Game {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}



impl Game {
    fn pick_random() -> Self {
        let rand = rand::rng().random_range(1..=3);
        match rand {
            1 => Game::Paper,
            2 => Game::Rock,
            3 => Game::Scissors,
            _ => unreachable!(),
        }
    }
}



pub fn play() {
    let mut user: Game;
    loop {
        print!("choose (rock/paper/scissors): ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        user = match input.trim().to_lowercase().as_str() {
            "scissors" =>  Game::Scissors,
            "rock" =>  Game::Rock,
            "paper" =>  Game::Paper,
            _ => {
                println!("please provide valid input");
                continue;
            }
        };
        break
    }
    let computer = Game::pick_random();
    let result = match (computer,user) {
        (Game::Scissors,Game::Rock) => GameResult::Win,
        (Game::Paper,Game::Scissors) => GameResult::Win,
        (Game::Rock,Game::Paper) => GameResult::Win,
        (a,b ) if a==b => GameResult::Draw,
        _ => GameResult::Lose
    };
    println!("{:?}",result)

}

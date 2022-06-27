use std::{
    fmt::Display,
    io::{self, Read},
};

use rand::{distributions::Standard, prelude::Distribution};

#[derive(Eq, Hash, PartialEq)]
enum GameAction {
    Invalid,
    Rock,
    Paper,
    Scissor,
}

impl GameAction {
    pub fn new(self, move_str: &str) -> GameAction {
        match &move_str.trim().to_lowercase() as &str {
            "rock" => GameAction::Rock,
            "paper" => GameAction::Paper,
            "scissor" => GameAction::Paper,
            _ => GameAction::Invalid,
        }
    }

    pub fn display_invalid_message() {
        println!("That's an invalid command. Please enter 'rock', 'paper', or 'scissors' only!");
    }
}

impl Distribution<GameAction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameAction {
        match rng.gen_range(0..=2) {
            0 => GameAction::Rock,
            1 => GameAction::Paper,
            2 => GameAction::Scissor,
            _ => panic!("Something went wrong in randomizing the computer's move!"),
        }
    }
}

impl Display for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GameAction::Invalid => write!(f, "Invalid"),
            GameAction::Rock => write!(f, "Rock"),
            GameAction::Paper => write!(f, "Paper"),
            GameAction::Scissor => write!(f, "Scissor"),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum DoExitAction {
    Invalid,
    Continue,
    Exit,
}

impl DoExitAction {
    pub fn new(self, exit_str: &str) -> DoExitAction {
        match &exit_str.trim().to_lowercase() as &str {
            "y" | "yes" => DoExitAction::Exit,
            "n" | "no" => DoExitAction::Continue,
            _ => DoExitAction::Invalid,
        }
    }

    pub fn display_invalid_message() {
        println!("That's an invalid command. Please enter 'y' or 'n' only!");
    }
}

fn main() {
    let is_running: bool = true;

    while is_running {
        // Initialize round
        let comp_mov: GameAction = rand::random();

        println!("{:?}", comp_mov.to_string());

        // // Get player's command
        // let mut cmd: String = String::new();
        // io::stdin().read_to_string(&mut cmd);
    }
}

use crate::lib::outcome::Outcome;
use rand::{distributions::Standard, prelude::Distribution};
use std::{fmt::Display, io};

#[derive(Eq, Hash, PartialEq, Debug)]
pub(crate) enum GameAction {
    Rock,
    Paper,
    Scissors,
}

impl GameAction {
    pub fn new(move_str: &str) -> Result<GameAction, io::Error> {
        match &move_str.trim().to_lowercase() as &str {
            "rock" => Ok(GameAction::Rock),
            "paper" => Ok(GameAction::Paper),
            "scissors" => Ok(GameAction::Scissors),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )),
        }
    }

    pub fn display_invalid_msg() {
        println!("Err...that's an invalid move. Please enter 'rock', 'paper', or 'scissors' only!");
    }

    pub fn beats(&self, other: &GameAction) -> Outcome {
        match self {
            GameAction::Rock => match other {
                GameAction::Rock => Outcome::Tie,
                GameAction::Paper => Outcome::Lose,
                GameAction::Scissors => Outcome::Win,
            },
            GameAction::Paper => match other {
                GameAction::Rock => Outcome::Win,
                GameAction::Paper => Outcome::Tie,
                GameAction::Scissors => Outcome::Lose,
            },
            GameAction::Scissors => match other {
                GameAction::Rock => Outcome::Lose,
                GameAction::Paper => Outcome::Win,
                GameAction::Scissors => Outcome::Tie,
            },
        }
    }
}

impl Distribution<GameAction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameAction {
        match rng.gen_range(0..=2) {
            0 => GameAction::Rock,
            1 => GameAction::Paper,
            2 => GameAction::Scissors,
            _ => panic!("Something went wrong in randomizing the computer's move!"),
        }
    }
}

impl Display for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameAction::Rock => write!(f, "rock"),
            GameAction::Paper => write!(f, "paper"),
            GameAction::Scissors => write!(f, "scissors"),
        }
    }
}

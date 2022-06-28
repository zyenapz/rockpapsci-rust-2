mod lib;

#[cfg(test)]
mod tests;

use crate::lib::{exit_action::ExitAction, game_action::GameAction, outcome::Outcome};
use std::io::{self, Write};

fn main() {
    let mut is_running: bool = true;
    let mut plyr_score: u32 = 0;
    let mut comp_score: u32 = 0;

    println!("Let's play Rock, Paper, Scissors!");

    while is_running {
        // Randomize computer's move
        let comp_mov: GameAction = rand::random();

        // Get player's move
        let mut plyr_mov: Option<GameAction> = None;

        while plyr_mov == None {
            print!("Pick your move: ");
            io::stdout()
                .flush()
                .expect("Error encountered when flushing");

            let mut mov_str: String = String::new();
            io::stdin()
                .read_line(&mut mov_str)
                .expect("Error encountered getting input");

            plyr_mov = if let Ok(ga) = GameAction::new(&mov_str) {
                Some(ga)
            } else {
                GameAction::display_invalid_msg();
                None
            };
        }

        // Compare moves
        let results: Outcome = plyr_mov.expect("Player move is None!").beats(&comp_mov);

        // Display results and score
        match results {
            Outcome::Lose => {
                println!("Ha! I picked {}! You LOSE!", comp_mov.to_string());
                comp_score += 1;
            }
            Outcome::Win => {
                println!("Argh, I picked {}! You WON!", comp_mov.to_string());
                plyr_score += 1;
            }
            Outcome::Tie => println!("We both picked {}. Let's try again!", comp_mov.to_string()),
        }

        println!("Score is {} - {}", plyr_score, comp_score);

        if results != Outcome::Tie {
            // Ask player if they want to play another round

            let mut exit_act = None;

            while exit_act == None {
                let mut exit_str: String = String::new();

                print!("Do you want to play again? (y/n): ");
                io::stdout()
                    .flush()
                    .expect("Error encountered when flushing");

                io::stdin()
                    .read_line(&mut exit_str)
                    .expect("Error encountered getting input");

                exit_act = if let Ok(ea) = ExitAction::new(&exit_str) {
                    Some(ea)
                } else {
                    ExitAction::display_invalid_msg();
                    None
                };
            }

            match exit_act.expect("Exit input is None!") {
                ExitAction::Continue => is_running = true,
                ExitAction::Exit => is_running = false,
            }
        }

        println!("");
    }

    println!("Bye! Thanks for playing with me! c:")
}

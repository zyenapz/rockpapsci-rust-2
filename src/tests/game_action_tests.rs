#[cfg(test)]
use std::io;

use crate::GameAction;

#[test]
fn test_gameaction_input_rock() {
    let s = "rock";
    assert_eq!(GameAction::new(s).unwrap(), GameAction::Rock);
}

#[test]
fn test_gameaction_input_paper() {
    let s = "paper";
    assert_eq!(GameAction::new(s).unwrap(), GameAction::Paper);
}

#[test]
fn test_gameaction_input_scissors() {
    let s = "scissors";
    assert_eq!(GameAction::new(s).unwrap(), GameAction::Scissors);
}

#[test]
fn test_gameaction_input_numbers() {
    let s = "1231";
    assert_eq!(
        GameAction::new(s).map_err(|e| e.kind()),
        Err(io::ErrorKind::InvalidInput)
    );
}

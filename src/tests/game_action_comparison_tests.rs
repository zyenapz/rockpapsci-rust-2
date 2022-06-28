#[cfg(test)]
use crate::{GameAction, Outcome};

#[test]
fn test_paper() {
    let act: GameAction = GameAction::Paper;

    let rock: GameAction = GameAction::Rock;
    let paper: GameAction = GameAction::Paper;
    let scissors: GameAction = GameAction::Scissors;

    assert_eq!(act.beats(&rock), Outcome::Win);
    assert_eq!(act.beats(&paper), Outcome::Tie);
    assert_eq!(act.beats(&scissors), Outcome::Lose);
}

#[test]
fn test_rock() {
    let act: GameAction = GameAction::Rock;

    let rock: GameAction = GameAction::Rock;
    let paper: GameAction = GameAction::Paper;
    let scissors: GameAction = GameAction::Scissors;

    assert_eq!(act.beats(&rock), Outcome::Tie);
    assert_eq!(act.beats(&paper), Outcome::Lose);
    assert_eq!(act.beats(&scissors), Outcome::Win);
}

#[test]
fn test_scissors() {
    let act: GameAction = GameAction::Scissors;

    let rock: GameAction = GameAction::Rock;
    let paper: GameAction = GameAction::Paper;
    let scissors: GameAction = GameAction::Scissors;

    assert_eq!(act.beats(&rock), Outcome::Lose);
    assert_eq!(act.beats(&paper), Outcome::Win);
    assert_eq!(act.beats(&scissors), Outcome::Tie);
}

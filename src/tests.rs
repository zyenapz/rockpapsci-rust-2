#[cfg(test)]
mod game_action_tests {
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
}

#[cfg(test)]
mod exit_action_tests {
    use crate::ExitAction;

    #[test]
    fn test_exit_input_yes() {
        let s1 = "yes";
        let s2 = "y";
        assert_eq!(ExitAction::new(s1).unwrap(), ExitAction::Continue);
        assert_eq!(ExitAction::new(s2).unwrap(), ExitAction::Continue);
    }

    #[test]
    fn test_exit_input_no() {
        let s1 = "no";
        let s2 = "n";
        assert_eq!(ExitAction::new(s1).unwrap(), ExitAction::Exit);
        assert_eq!(ExitAction::new(s2).unwrap(), ExitAction::Exit);
    }
}

#[cfg(test)]
mod game_action_comparison_tests {
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
}

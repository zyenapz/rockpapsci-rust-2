#[cfg(test)]
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

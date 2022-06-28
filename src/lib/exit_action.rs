use std::io;

#[derive(Eq, Hash, PartialEq, Debug)]
pub(crate) enum ExitAction {
    Continue,
    Exit,
}

impl ExitAction {
    pub fn new(exit_str: &str) -> Result<ExitAction, io::Error> {
        match &exit_str.trim().to_lowercase() as &str {
            "y" | "yes" => Ok(ExitAction::Continue),
            "n" | "no" => Ok(ExitAction::Exit),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )),
        }
    }

    pub fn display_invalid_msg() {
        println!("Err...that's an invalid command. Please enter 'y' or 'n' only!");
    }
}

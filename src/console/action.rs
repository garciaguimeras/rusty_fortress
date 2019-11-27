use std::fmt;

pub enum Action {
    Error,
    Quit,
    Help,
    Other
}

impl fmt::Display for Action {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Error => write!(f, "Action::Error"),
            Action::Other => write!(f, "Action::Other"),
            Action::Help => write!(f, "Action::Help"),
            Action::Quit => write!(f, "Action::Quit")
        }
    }

}
use std::fmt;
use crate::app::parser::Keyword;

pub struct Command {
    pub status: bool,
    pub keyword: Option<Keyword>,
    pub objects: Vec<String>
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return if self.status {
            let kwd = self.keyword.as_ref().unwrap();
            if self.objects.len() == 0 {
                write!(f, "Keyword: {}", kwd)
            }
            else {
                let mut text = String::from("");
                self.objects.iter().for_each(|s| {
                    text = if text == "" { format!("'{}'", s) } else { format!("{}, '{}'", text, s) }
                });
                write!(f, "Keyword: {} / Objects: {}", kwd, text)
            }
        }
        else {
            write!(f, "No keyword defined")
        }
    }
}

pub trait HandleCommand {
    fn resolve_command(&mut self, command: &Command) -> String;
}

pub struct DefaultCommandHandler {}

impl DefaultCommandHandler {
    pub fn new() -> DefaultCommandHandler {
        DefaultCommandHandler {}
    }
}

impl HandleCommand for DefaultCommandHandler {

    fn resolve_command(&mut self, command: &Command) -> String { 
        match &command.keyword {
            Option::None => String::from("Don't understand what you want to do?"),
            Option::Some(keyword) => match keyword {
                Keyword::GoThrough => String::from("Can't go there"),
                Keyword::Help => String::from("Get a little help to my friends"),
                Keyword::Open => String::from("Don't know how to open that"),
                Keyword::OpenWith => String::from("Don't know how to open that"),
                Keyword::Quit => String::from("Goodbye cruel world!"),
                Keyword::Take => String::from("Can't take that"),
                Keyword::View => String::from("Can't see anything")
            }
        }
    }

}
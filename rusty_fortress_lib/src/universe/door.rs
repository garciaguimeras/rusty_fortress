use crate::app::command::DefaultCommandHandler;
use crate::app::command::Command;
use crate::app::command::HandleCommand;
use crate::app::parser::Keyword;
use crate::universe::base::Open;
use crate::universe::base::OpenWith;

#[derive(Clone, PartialEq)]
enum DoorState {
    OPENED,
    CLOSED
}

// Door
#[derive(Clone, PartialEq)]
pub struct Door {
    state: DoorState
}

impl HandleCommand for Door {   
    fn resolve_command(&mut self, command: &Command) -> String { 
        match command.keyword {
            Option::Some(Keyword::Open) => {
                self.open()
            },
            _ => {
                let mut handler = DefaultCommandHandler::new();
                let text = handler.resolve_command(&command);
                text
            }
        }
    }
}

impl Open for Door {
    fn open(&mut self) -> String { 
        return if self.state == DoorState::CLOSED {
            self.state = DoorState::OPENED;
            String::from("You open the door")
        }
        else {
            String::from("The door is already opened")   
        }
    }
}

impl Door {
    pub fn new() -> Door {
        Door {
            state: DoorState::CLOSED
        }
    }
}

// Locked door
#[derive(Clone, PartialEq)]
pub struct LockedDoor {
    state: DoorState,
    key: String
}

impl HandleCommand for LockedDoor {   
    fn resolve_command(&mut self, command: &Command) -> String { 
        match command.keyword {
            Option::Some(Keyword::Open) => {
                self.open()
            },
            Option::Some(Keyword::OpenWith) => {
                let default_key = String::from("");
                let key = command.objects.get(0).unwrap_or(&default_key);
                self.open_with(key)
            },
            _ => {
                let mut handler = DefaultCommandHandler::new();
                let text = handler.resolve_command(&command);
                text
            }
        }
    }
}

impl OpenWith for LockedDoor {
    fn open_with(&mut self, key: &str) -> String { 
        return if self.state == DoorState::CLOSED {
            if self.key == key {
                self.state = DoorState::OPENED;
                String::from("You open the door")
            }
            else {
                String::from("The door is locked. Try another key")
            }
        }
        else {
            String::from("The door is already opened")   
        }
    }
}

impl Open for LockedDoor {
    fn open(&mut self) -> String { 
        return if self.state == DoorState::CLOSED {
            String::from("The door is locked")
        }
        else {
            String::from("The door is already opened")   
        }
    }
}

impl LockedDoor {
    pub fn new(key: &str) -> LockedDoor {
        LockedDoor {
            state: DoorState::CLOSED,
            key: String::from(key)
        }
    }
}
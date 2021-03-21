use crate::app::command::DefaultCommandHandler;
use crate::app::command::Command;
use crate::app::command::HandleCommand;
use crate::app::parser::Keyword;
use crate::universe::base::Open;

#[derive(Clone, PartialEq)]
enum DoorState {
    OPENED,
    CLOSED
}

#[derive(Clone, PartialEq)]
pub struct Door {
    state: DoorState
}

impl HandleCommand for Door {   
    fn resolve_command(&mut self, command: &Command) -> String { 
        return if let Option::Some(Keyword::Open) = command.keyword {
            self.open()
        }
        else { 
            let mut handler = DefaultCommandHandler::new();
            let text = handler.resolve_command(&command);
            text
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
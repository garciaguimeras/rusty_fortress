use crate::app::command::DefaultCommandHandler;
use crate::app::command::HandleCommand;
use crate::app::parser::Keyword;
use crate::app::parser::StateMachine;

pub trait InOut {
    fn read_line(&self) -> String;
    fn write_line(&self, line: String);
}

pub fn run<T: InOut>(in_out: T) {
    let mut handler = DefaultCommandHandler::new();
    let state_machine = StateMachine::build();
    let mut running = true;
    while running {
        let line = in_out.read_line();
        if line.len() > 0 {
            let command = state_machine.parse_line(&line);
            //println!("{}", command);
            
            if let Option::Some(Keyword::Quit) = command.keyword {
                running = false;
            } 
            let text = handler.resolve_command(&command);
            in_out.write_line(text);
        }
    }
}
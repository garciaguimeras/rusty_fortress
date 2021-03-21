use crate::app::parser;

pub trait InOut {
    fn read_line(&self) -> String;
    fn write_line(&self, line: String);
}

pub fn run<T: InOut>(in_out: T) {
    let state_machine = parser::StateMachine::build();
    let mut running = true;
    while running {
        let line = in_out.read_line();
        if line.len() > 0 {
            let output = state_machine.parse_line(&line);
            let last_action = output.get(output.len() - 1).unwrap();
            let result = match last_action { 
                parser::OutputAction::Keyword(parser::Keyword::Quit) => { 
                    (false, String::from("Good bye, cruel world!"))
                },
                _ => {
                    (true, String::from("Keep trying to quit"))
                }
            };
            running = result.0;
            in_out.write_line(result.1);
        }
    }
}
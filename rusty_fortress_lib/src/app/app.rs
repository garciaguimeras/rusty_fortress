use crate::app::parser;
use crate::app::env;

pub trait InOut {
    fn read_line(&self) -> String;
    fn write_line(&self, line: String);
}

pub fn run<T: InOut>(in_out: T) {
    let state_machine = parser::StateMachine::build();
    let mut environment = env::Environment::new();
    let mut running = true;
    while running {
        let line = in_out.read_line();
        if line.len() > 0 {
            let output = state_machine.parse_line(&line);
            let result = environment.execute(&output);
            running = result.0;
            in_out.write_line(result.1);
        }
    }
}
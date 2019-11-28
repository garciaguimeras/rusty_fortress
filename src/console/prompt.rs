use std::io;
use std::io::Write;
use crate::console::parser;
use crate::cmd;

fn prompt() {
    println!();
    println!("What do you want to do now?");
    print!("> ");
    let _ = io::stdout().flush();
}

fn read_line() -> String {
    let mut line = String::new();
    prompt();    
    match io::stdin().read_line(&mut line) {
        Ok(_) => {},
        Err(_) => line = String::new()
    }

    line.trim().to_string()
}

pub fn run() {
    let state_machine = parser::StateMachine::build();
    let mut running = true;
    while running {
        let line = read_line();
        if line.len() > 0 {
            let output = state_machine.parse_line(&line);
            running = cmd::execute(&output);
        }
    }
}
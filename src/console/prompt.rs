use std::io;
use std::io::Write;

use crate::console::action::Action;
use crate::console::parser;
use crate::obj;
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

fn get_action(output: &Vec<parser::OutputAction>) -> Action {
    // Check if last output is error
    let last_action = output.get(output.len() - 1).unwrap();
    if let parser::OutputAction::Error = last_action {
        return Action::Error;
    }

    // Check for first action (should be a command)
    let first_action = output.get(0).unwrap();
    if let parser::OutputAction::Keyword(k) = first_action {
        if k == "exit" || k == "quit" {
            return Action::Quit;
        }
        if k == "help" || k == "?" {
            return Action::Help;
        }
        return Action::Other;
    }

    // Else, error...
    Action::Error
}

fn print_help() {
    println!("This help, right now, doesn't help too much.");
}

fn print_error() {
    println!("Cannot understand what are you trying to do.");
}

fn check_commands(executor: &cmd::Executor, action_str: &str) {
    let black_arrow = obj::base::Arrow { 
        name: String::from("Black arrow"),
        description: String::from("It's a very black arrow")
    };
    executor.execute_command(&black_arrow, action_str);
}

pub fn run() {
    let state_machine = parser::StateMachine::build();
    let executor = cmd::Executor::new();
    let mut running = true;
    while running {
        let line = read_line();
        if line.len() > 0 {
            let output = state_machine.parse_line(&line);
            let action = get_action(&output);
            match action {
                Action::Quit => running = false,
                Action::Help => print_help(),
                Action::Error => print_error(),
                Action::Other => {
                    // check_commands(&executor, &words[0]);
                }
            };
        }
    }
}
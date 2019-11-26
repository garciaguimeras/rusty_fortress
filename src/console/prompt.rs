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

    line
}

fn split_line(line: &str) -> Vec<String> {
    let splitted: Vec<&str> = line.trim().split(' ').collect();
    let mut words: Vec<String> = Vec::new(); 
    for word in splitted.iter() {
        words.push(String::from(*word));
    }
    words
}

fn get_action(first_word: &str) -> Action {
    let word = String::from(first_word);

    if word == "" {
        return Action::None;
    }

    if word == "exit" || word == "quit" {
        return Action::Quit;
    }

    if word == "help" || word == "?" {
        return Action::Help;
    }

    Action::Other
}

fn print_help() {
    println!("This help, right now, doesn't help too much.");
}

fn check_commands(executor: &cmd::Executor, action_str: &str) {
    let black_arrow = obj::base::Arrow { 
        name: String::from("Black arrow"),
        description: String::from("It's a very black arrow")
    };
    executor.execute_command(&black_arrow, action_str);
}

fn parse_line(line: &str) {
    let state_machine = parser::StateMachine::create();
    state_machine.parse_line(line);

}

pub fn run() {
    let executor = cmd::Executor::init();
    let mut running = true;
    while running {
        let line = read_line();
        let words = split_line(&line);
        if words.len() > 0 {
            let action = get_action(&words[0]);
            match action {
                Action::Quit => running = false,
                Action::Help => print_help(),
                Action::None => {},
                Action::Other => {
                    //check_commands(&executor, &words[0]);
                    parse_line(&line);
                }
            };
        }
    }
}
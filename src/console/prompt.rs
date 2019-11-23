use std::io;
use std::io::Write;

use crate::console::action::Action;
use crate::obj;
use crate::cmd;

fn prompt() {
    println!();
    print!("> ");
    let _ = io::stdout().flush();
}

fn read_line_and_split() -> Vec<String> {
    let mut line = String::new();
    prompt();    
    match io::stdin().read_line(&mut line) {
        Ok(_) => {},
        Err(_) => line = String::new()
    }

    let split_line: Vec<&str> = line.trim().split(' ').collect();
    let mut words: Vec<String> = Vec::new(); 
    for word in split_line.iter() {
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

pub fn run() {
    let executor = cmd::Executor::init();
    let mut running = true;
    while running {
        let words = read_line_and_split();
        if words.len() > 0 {
            let action = get_action(&words[0]);
            match action {
                Action::Quit => running = false,
                Action::Help => print_help(),
                Action::None => {},
                Action::Other => {
                    check_commands(&executor, &words[0]);
                }
            };
        }
    }
}
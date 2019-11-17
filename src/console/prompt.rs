use std::io;
use std::io::Write;
use super::action::Action;

fn prompt() {
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

    if word == "salir" {
        return Action::Quit;
    }

    if word == "ayuda" || word == "?" {
        return Action::Help;
    }

    Action::Other
}

fn print_help() {
    println!("Esta es una ayuda que (de momento) no parece de mucha ayuda.");
}

pub fn run() {
    let mut running = true;
    while running {
        let words = read_line_and_split();
        if words.len() > 0 {
            let action = get_action(&words[0]);
            match action {
                Action::Quit => running = false,
                Action::Help => print_help(),
                _ => println!("{0}", action)
            };
        }
    }
}
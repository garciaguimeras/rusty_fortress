use crate::obj::base;
use crate::obj::env;
use super::parser;

fn print_help() {
    println!("This help, right now, doesn't help too much.");
}

fn print_error() {
    println!("Cannot understand what are you trying to do.");
}

fn print_quit() {
    println!("Good bye, cruel world.");
}

fn open(environment: &env::Environment, objects: &Vec<String>) {
    println!("cmd: open");
}

fn open_with(environment: &env::Environment, objects: &Vec<String>) {
    println!("cmd: open with");
}

fn get_all_keywords(output: &Vec<parser::OutputAction>) -> Vec<parser::Keyword> {
    let mut keywords: Vec<parser::Keyword> = Vec::new();
    output.iter().for_each(|o| { 
        match o {
            parser::OutputAction::Keyword(k) => { keywords.push(k.clone()) },
            _ => {}
        };
    });
    keywords
}

fn get_all_objects(output: &Vec<parser::OutputAction>) -> Vec<String> {
    let mut objects: Vec<String> = Vec::new();
    output.iter().for_each(|o| { 
        match o {
            parser::OutputAction::Object(txt) => { objects.push(txt.clone()) },
            _ => {}
        };
    });
    objects
}

pub fn execute(environment: &env::Environment, output: &Vec<parser::OutputAction>) -> bool {
    // Check if last output is error
    let last_action = output.get(output.len() - 1).unwrap();
    if let parser::OutputAction::Error = last_action {
        print_error();
        return true;
    }
 
    let keywords = get_all_keywords(&output);
    let objects = get_all_objects(&output);
    return match keywords.as_slice() {
        [parser::Keyword::Quit] => {
            print_quit();
            false
        },
        [parser::Keyword::Help] => {
            print_help();
            true
        },
        [parser::Keyword::Open] => {
            open(environment, &objects);
            true
        },
        [parser::Keyword::Open, parser::Keyword::With] => {
            open_with(environment, &objects);
            true
        },
        _ => { 
            print_error();
            true
        }
    };
}
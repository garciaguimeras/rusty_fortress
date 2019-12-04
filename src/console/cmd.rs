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
    match objects.get(0) {
        Some(obj_name) => {
            match environment.find_object_by_name(&obj_name) {
                Some(obj) => {
                    let response = obj.open(&environment);
                    println!("{}", response);
                },
                _ => println!("Cannot find {}.", obj_name)
            }
        },
        _ => println!("Don't know what do you want to open.")
    }
}

fn open_with(environment: &env::Environment, objects: &Vec<String>) {
    match (objects.get(0), objects.get(1)) {
        (Some(obj_name1), Some(obj_name2)) => {
            match (environment.find_object_by_name(&obj_name1), environment.find_object_by_name(&obj_name2))  {
                (Some(obj1), Some(obj2)) => {
                    let response = obj1.open_with(&environment, obj2);
                    println!("{}", response);
                },
                (Some(_), None) => println!("Cannot find {}.", obj_name2),
                _ => println!("Cannot find {}.", obj_name1)
            }
        },
        (Some(_), None) => println!("Don't know what do you want to open with."),
        _ => println!("Don't know what do you want to open."),
    }
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
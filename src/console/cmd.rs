use crate::obj::env;
use crate::obj::base;
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

fn one_obj_fn<P>(environment: &env::Environment, objects: &Vec<String>, null_obj_err: &str, predicate: P)
where P: Fn(&env::Environment, &Box<base::BaseObject>) -> String {
    match objects.get(0) {
        Some(obj_name) => {
            match environment.find_object_by_name(&obj_name) {
                Some(obj) => {
                    let response = predicate(&environment, obj);
                    println!("{}", response);
                },
                _ => println!("Cannot find {}.", obj_name)
            }
        },
        _ => println!("{}", null_obj_err)
    }
}

fn two_objs_fn<P>(environment: &env::Environment, objects: &Vec<String>, null_obj1_err: &str, null_obj2_err: &str, predicate: P) 
where P: Fn(&env::Environment, &Box<base::BaseObject>, &Box<base::BaseObject>) -> String {    
    match (objects.get(0), objects.get(1)) {
        (Some(obj_name1), Some(obj_name2)) => {
            match (environment.find_object_by_name(&obj_name1), environment.find_object_by_name(&obj_name2))  {
                (Some(obj1), Some(obj2)) => {
                    let response = predicate(&environment, obj1, obj2);
                    println!("{}", response);
                },
                (Some(_), None) => println!("Cannot find {}.", obj_name2),
                _ => println!("Cannot find {}.", obj_name1)
            }
        },
        (Some(_), None) => println!("{}", null_obj2_err),
        _ => println!("{}", null_obj1_err),
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
            one_obj_fn(environment, 
                &objects, 
                "Don't know what do you want to open.", 
                |env, obj| { obj.open(env).to_string() }
            );
            true
        },
        [parser::Keyword::Open, parser::Keyword::With] => {
            two_objs_fn(environment, 
                &objects, 
                "Don't know what do you want to open.", 
                "Don't know what do you want to open with.", 
                |env, obj1, obj2| { obj1.open_with(env, obj2).to_string() }
            );
            true
        },
        [parser::Keyword::View] => {
            one_obj_fn(environment, 
                &objects, 
                "Don't know what do you want to view.", 
                |env, obj| { obj.view(env).to_string() }
            );            
            true
        },
        [parser::Keyword::Take] => {
            one_obj_fn(environment, 
                &objects, 
                "Don't know what do you want to take.", 
                |env, obj| { obj.take(env).to_string() }
            );  
            true
        },
        _ => { 
            print_error();
            true
        }
    };
}
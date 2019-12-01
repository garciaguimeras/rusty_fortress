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

fn view(obj: &base::BaseObject) -> &str {
    obj.view()
}

fn open(obj: &base::BaseObject) -> &str {
    obj.open()
}

fn take(obj: &base::BaseObject) -> &str {
    obj.take()
}

fn execute_command(output: &Vec<parser::OutputAction>) {

}

pub fn execute(environment: &env::Environment, output: &Vec<parser::OutputAction>) -> bool {
    // Check if last output is error
    let last_action = output.get(output.len() - 1).unwrap();
    if let parser::OutputAction::Error = last_action {
        print_error();
        return true;
    }

    // Check for first action (should be a command)
    let first_action = output.get(0).unwrap();
    if let parser::OutputAction::Keyword(k) = first_action {
        return match k {
            parser::Keyword::Quit => {
                print_quit();
                false
            },
            parser::Keyword::Help => {
                print_help();
                true
            },
            _ => {
                execute_command(output);
                true
            }
        }       
    }

    // Else, error...
    print_error();
    true
}
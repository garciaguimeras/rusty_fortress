use crate::obj::base::*;
use crate::console::parser;

fn print_help() {
    println!("This help, right now, doesn't help too much.");
}

fn print_error() {
    println!("Cannot understand what are you trying to do.");
}

fn print_quit() {
    println!("Good bye, cruel world.");
}

fn view(obj: &BaseObject) -> &str {
    obj.view()
}

fn open(obj: &BaseObject) -> &str {
    obj.open()
}

fn take(obj: &BaseObject) -> &str {
    obj.take()
}

fn execute_command(output: &Vec<parser::OutputAction>) {

}

pub fn execute(output: &Vec<parser::OutputAction>) -> bool {
    // Check if last output is error
    let last_action = output.get(output.len() - 1).unwrap();
    if let parser::OutputAction::Error = last_action {
        print_error();
        return true;
    }

    // Check for first action (should be a command)
    let first_action = output.get(0).unwrap();
    if let parser::OutputAction::Keyword(k) = first_action {

        // Predefined actions
        if k == "exit" || k == "quit" {
            print_quit();
            return false;
        }
        if k == "help" || k == "?" {
            print_help();
            return true;
        }

        // Commands
        execute_command(output);
        return true;
    }

    // Else, error...
    print_error();
    true
}
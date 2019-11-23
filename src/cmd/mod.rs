use std::collections::HashMap;
use crate::obj::base::*;

type CommandFn = fn(&BaseObject) -> &str; 

fn view(obj: &BaseObject) -> &str {
    obj.view()
}

fn open(obj: &BaseObject) -> &str {
    obj.open()
}

fn take(obj: &BaseObject) -> &str {
    obj.take()
}

pub struct Executor {
    map: HashMap<String, CommandFn>
}

impl Executor {

    pub fn init() -> Executor {
        let mut map: HashMap<String, CommandFn> = HashMap::new();

        map.insert(String::from("view"), view);
        map.insert(String::from("open"), open);
        map.insert(String::from("take"), take);
        
        Executor {
            map: map
        }
    }

    pub fn execute_command(&self, obj: &BaseObject, command: &str) {
        let response = match self.map.get(&String::from(command)) {
            Some(f) => f(obj),
            None => "Hmmm... Don't know what are you trying to do"
        };
        println!("{}", response);
    }

}
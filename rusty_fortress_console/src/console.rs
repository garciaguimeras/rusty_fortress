use std::io;
use std::io::Write;
use rusty_fortress_lib::app::app;

pub struct Console {}

impl Console {

    pub fn new() -> Console {
        Console {}
    }

    fn prompt(&self) {
        println!();
        //println!("What do you want to do now?");
        print!("> ");
        let _ = io::stdout().flush();
    }

}

impl app::InOut for Console {

    fn read_line(&self) -> String {
        let mut line = String::new();
        self.prompt();    
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(_) => line = String::new()
        }
        line.trim().to_string()
    }

    fn write_line(&self, line: String) {
        println!("{}", line);
    }

}

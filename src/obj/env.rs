use super::base;
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

pub struct Environment {
    //main_character: Box<dyn base::BaseObject>,
    objects: Vec<Box<dyn base::BaseObject>>
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            objects: vec!(base::Door::boxed( 
                String::from("Main Door"),
                String::from("It's just a main door"),
                true,
                Option::None
            ))
        }
    }

    fn one_obj_fn<P>(&mut self, objects: &Vec<String>, null_obj_err: &str, predicate: P)
    where P: Fn(&mut Box<dyn base::BaseObject>) -> String {
        match objects.get(0) {
            Some(obj_name) => {
                
                let cmp_name = &obj_name.to_lowercase();
                let pos = self.objects.iter().position(|o| { o.name().to_lowercase() == *cmp_name });

                match pos {
                    Option::Some(idx) => {
                        let mut boxed = self.objects.get_mut(idx).unwrap();
                        let response = predicate(&mut boxed);
                        println!("{}", response);
                    },
                    _ => println!("Cannot find {}.", obj_name)
                }
            },
            _ => println!("{}", null_obj_err)
        }
    }

    fn two_objs_fn<P>(&mut self, objects: &Vec<String>, null_obj1_err: &str, null_obj2_err: &str, predicate: P) 
    where P: Fn(&mut Box<dyn base::BaseObject>, &Box<dyn base::BaseObject>) -> String {    
        match (objects.get(0), objects.get(1)) {
            (Some(obj_name1), Some(obj_name2)) => {
                
                let cmp_name1 = &obj_name1.to_lowercase();
                let cmp_name2 = &obj_name2.to_lowercase();
                let pos1 = self.objects.iter().position(|o| { o.name().to_lowercase() == *cmp_name1 });
                let pos2 = self.objects.iter().position(|o| { o.name().to_lowercase() == *cmp_name2 });

                match (pos1, pos2) {
                    (Option::Some(idx1), Option::Some(idx2)) => {
                        let boxed1 = self.objects.get(idx1).unwrap();
                        let mut obj1 = (*boxed1).clone();
                        let boxed2 = self.objects.get(idx2).unwrap();
                        let mut obj2 = (*boxed2).clone();

                        let response = predicate(&mut obj1, &mut obj2);
                        
                        let mut mut_obj = self.objects.get_mut(idx1).unwrap();
                        *mut_obj = obj1;
                        mut_obj = self.objects.get_mut(idx2).unwrap();
                        *mut_obj = obj2;
                        
                        println!("{}", response);
                    },
                    (Option::Some(_), Option::None) => println!("Cannot find {}.", obj_name2),
                    _ => println!("Cannot find {}.", obj_name1)
                }
            },
            (Some(_), None) => println!("{}", null_obj2_err),
            _ => println!("{}", null_obj1_err),
        }
    }

    fn get_all_keywords(&self, output: &Vec<parser::OutputAction>) -> Vec<parser::Keyword> {
        let mut keywords: Vec<parser::Keyword> = Vec::new();
        output.iter().for_each(|o| { 
            match o {
                parser::OutputAction::Keyword(k) => { keywords.push(k.clone()) },
                _ => {}
            };
        });
        keywords
    }
    
    fn get_all_objects(&self, output: &Vec<parser::OutputAction>) -> Vec<String> {
        let mut objects: Vec<String> = Vec::new();
        output.iter().for_each(|o| { 
            match o {
                parser::OutputAction::Object(txt) => { objects.push(txt.clone()) },
                _ => {}
            };
        });
        objects
    }

    pub fn execute(&mut self, output: &Vec<parser::OutputAction>) -> bool {
        // Check if last output is error
        let last_action = output.get(output.len() - 1).unwrap();
        if let parser::OutputAction::Error = last_action {
            print_error();
            return true;
        }
     
        let keywords = self.get_all_keywords(&output);
        let objects = self.get_all_objects(&output);
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
                self.one_obj_fn(&objects, 
                    "What do you want to open?", 
                    |obj| { 
                        return match obj.as_open_mut() {
                            Some(open) => open.open().to_string(),
                            None => "Oops! Cannot open that".to_string()
                        }
                    }
                );
                true
            },
            [parser::Keyword::Open, parser::Keyword::With] => {
                self.two_objs_fn(&objects, 
                    "What do you want to open?", 
                    "What do you want to use to open?", 
                    |obj1, obj2| { 
                        return match obj1.as_open_mut() {
                            Some(open) => open.open_with(obj2).to_string(),
                            None => "Oops! Cannot open that".to_string()
                        }
                    }
                );
                true
            },
            [parser::Keyword::View] => {
                self.one_obj_fn(&objects, 
                    "What do you want to view?", 
                    |obj| { 
                        return match obj.as_view_mut() {
                            Some(view) => view.view().to_string(),
                            None => "Oops! Cannot view that. It seems to be invisible!".to_string()
                        };
                    }
                );            
                true
            },
            [parser::Keyword::Take] => {
                self.one_obj_fn(&objects, 
                    "What do you want to take?", 
                    |obj| {
                        return match obj.as_take_mut() {
                            Some(take) => take.take().to_string(),
                            None => "Oops! Cannot take that".to_string()
                        }
                    }
                );  
                true
            },
            [parser::Keyword::GoThrough] => {
                self.one_obj_fn(&objects, 
                    "What do you want to go through?", 
                    |obj| { 
                        return match obj.as_go_mut() {
                            Some(go) => go.go_through().to_string(),
                            None => "Oops! Cannot go through that".to_string()
                        }
                    }
                );
                true
            },
            _ => { 
                print_error();
                true
            }
        };
    }
}
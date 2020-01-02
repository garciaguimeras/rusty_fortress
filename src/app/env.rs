use crate::obj::base;
use super::parser;

fn help_message() -> String {
    "This help, right now, doesn't help too much.".to_string()
}

fn error_message() -> String {
    "Cannot understand what are you trying to do.".to_string()
}

fn quit_message() -> String {
    "Good bye, cruel world.".to_string()
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

    fn one_obj_fn<P>(&mut self, objects: &Vec<String>, null_obj_err: &str, predicate: P) -> String
    where P: Fn(&mut Box<dyn base::BaseObject>) -> String {
        match objects.get(0) {
            Some(obj_name) => {
                
                let cmp_name = &obj_name.to_lowercase();
                let pos = self.objects.iter().position(|o| { o.name().to_lowercase() == *cmp_name });

                match pos {
                    Option::Some(idx) => {
                        let mut boxed = self.objects.get_mut(idx).unwrap();
                        let response = predicate(&mut boxed);
                        format!("{}", response)
                    },
                    _ => format!("Cannot find {}.", obj_name)
                }
            },
            _ => format!("{}", null_obj_err)
        }
    }

    fn two_objs_fn<P>(&mut self, objects: &Vec<String>, null_obj1_err: &str, null_obj2_err: &str, predicate: P) -> String 
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
                        
                        format!("{}", response)
                    },
                    (Option::Some(_), Option::None) => format!("Cannot find {}.", obj_name2),
                    _ => format!("Cannot find {}.", obj_name1)
                }
            },
            (Some(_), None) => format!("{}", null_obj2_err),
            _ => format!("{}", null_obj1_err)
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

    pub fn execute(&mut self, output: &Vec<parser::OutputAction>) -> (bool, String) {
        // Check if last output is error
        let last_action = output.get(output.len() - 1).unwrap();
        if let parser::OutputAction::Error = last_action {
            return (true, error_message());
        }
     
        let keywords = self.get_all_keywords(&output);
        let objects = self.get_all_objects(&output);
        return match keywords.as_slice() {
            [parser::Keyword::Quit] => {
                (false, quit_message())
            },
            [parser::Keyword::Help] => {
                (true, help_message())
            },
            [parser::Keyword::Open] => {
                (true, self.one_obj_fn(&objects, 
                    "What do you want to open?", 
                    |obj| { 
                        return match obj.into_boxed_mut() as Option<Box<&mut base::Open>> {
                            Some(open) => open.open().to_string(),
                            None => "Oops! Cannot open that".to_string()
                        }
                    }
                ))
            },
            [parser::Keyword::Open, parser::Keyword::With] => {
                (true, self.two_objs_fn(&objects, 
                    "What do you want to open?", 
                    "What do you want to use to open?", 
                    |obj1, obj2| { 
                        return match obj1.into_boxed_mut() as Option<Box<&mut base::Open>> {
                            Some(open) => open.open_with(obj2).to_string(),
                            None => "Oops! Cannot open that".to_string()
                        }
                    }
                ))
            },
            [parser::Keyword::View] => {
                (true, self.one_obj_fn(&objects, 
                    "What do you want to view?", 
                    |obj| { 
                        return match obj.into_boxed_mut() as Option<Box<&mut base::View>> {
                            Some(view) => view.view().to_string(),
                            None => "Oops! Cannot view that. It seems to be invisible!".to_string()
                        };
                    }
                ))
            },
            [parser::Keyword::Take] => {
                (true, self.one_obj_fn(&objects, 
                    "What do you want to take?", 
                    |obj| {
                        return match obj.into_boxed_mut() as Option<Box<&mut base::Take>> {
                            Some(take) => take.take().to_string(),
                            None => "Oops! Cannot take that".to_string()
                        }
                    }
                ))
            },
            [parser::Keyword::GoThrough] => {
                (true, self.one_obj_fn(&objects, 
                    "What do you want to go through?", 
                    |obj| { 
                        return match obj.into_boxed_mut() as Option<Box<&mut base::Go>> {
                            Some(go) => go.go_through().to_string(),
                            None => "Oops! Cannot go through that".to_string()
                        }
                    }
                ))
            },
            _ => { 
                (true, error_message())
            }
        };
    }
}
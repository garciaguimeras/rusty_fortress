pub trait BaseObject {

    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn clone(&self) -> Box<BaseObject>;

    fn view(&self) -> &str {
        self.description()
    }

    fn open(&mut self) -> &str {
        "Oops! Don't know how to open that."
    }

    fn open_with(&mut self, _obj: &Box<BaseObject>) -> &str {
        "Oops! Don't know how to open that."
    }

    fn take(&self) -> &str {
        "Oops! Cannot take that."
    }

    fn go_through(&self) -> &str {
        "Oops! Cannot go through that."
    }

}

#[derive(Clone)]
pub struct Door {
    pub name: String,
    pub description: String,
    pub is_locked: bool,
    pub locked_by: Option<String>
}

impl Door {

    pub fn boxed(name: String, description: String, is_locked: bool, locked_by: Option<String>) -> Box<Door> {
        Box::new(Door {
            name: name,
            description: description,
            is_locked: is_locked,
            locked_by: locked_by
        })
    }

}

impl BaseObject for Door {

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn clone(&self) -> Box<BaseObject> {
        Door::boxed(self.name.clone(), 
            self.description.clone(), 
            self.is_locked.clone(), 
            self.locked_by.clone())
    }

    fn open(&mut self) -> &str {
        return match self.is_locked {
            false => "It's already opened.",
            true => {
                return match self.locked_by {
                    Option::None => { 
                        self.is_locked = false;
                        "You open the door."
                    },
                    Option::Some(_) => "You need some key to open that door."
                }
            }
        }
    }

} 
// View

pub trait AsView {
    fn as_view_mut(&mut self) -> Option<Box<&mut dyn View>> {
        Option::None
    }
}

pub trait View {
    fn view(&self) -> &str;
}

impl<T: View> AsView for T {
    fn as_view_mut(&mut self) -> Option<Box<&mut dyn View>> {
        Option::Some(Box::new(self))
    }
}

// Open

pub trait AsOpen {
    fn as_open_mut(&mut self) -> Option<Box<&mut dyn Open>> {
        Option::None
    }
}

pub trait Open {
    fn open(&mut self) -> &str;
    fn open_with(&mut self, _obj: &Box<BaseObject>) -> &str;
}

impl<T: Open> AsOpen for T {
    fn as_open_mut(&mut self) -> Option<Box<&mut dyn Open>> {
        Option::Some(Box::new(self))
    }
}

// Take

pub trait AsTake {
    fn as_take_mut(&mut self) -> Option<Box<&mut dyn Take>> {
        Option::None
    }
}

pub trait Take {
    fn take(&self) -> &str;
}

impl<T: Take> AsTake for T {
    fn as_take_mut(&mut self) -> Option<Box<&mut dyn Take>> {
        Option::Some(Box::new(self))
    }
}

// Take

pub trait AsGo {
    fn as_go_mut(&mut self) -> Option<Box<&mut dyn Go>> {
        Option::None
    }
}

pub trait Go {
    fn go_through(&self) -> &str;
}

impl<T: Go> AsGo for T {
    fn as_go_mut(&mut self) -> Option<Box<&mut dyn Go>> {
        Option::Some(Box::new(self))
    }
}


// BaseObject

pub trait BaseObject: AsView + AsOpen + AsTake + AsGo {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn clone(&self) -> Box<BaseObject>;
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

impl AsTake for Door {}

impl Go for Door {
    fn go_through(&self) -> &str {
        "## Not implemented yet ##"
    }
}

impl View for Door {
    fn view(&self) -> &str {
        self.description()
    }
}

impl Open for Door {
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

    fn open_with(&mut self, obj: &Box<BaseObject>) -> &str {
        return match self.is_locked {
            false => "It's already opened.",
            true => {
                return match &self.locked_by {
                    Option::None => "You don't need a key to unlock that door.",
                    Option::Some(key) => {
                        if key == obj.name() { "You open the door." } else { "This is not the right key to unlock the door." }
                    }
                }
            }
        }
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

} 
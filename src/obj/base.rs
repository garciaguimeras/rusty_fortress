pub trait IntoBoxed<T: ?Sized> {
    fn into_boxed(&self) -> Option<Box<&T>>;
    fn into_boxed_mut(&mut self) -> Option<Box<&mut T>>;
}

// View

pub trait View {
    fn view(&self) -> &str;
}

// Open

pub trait Open {
    fn open(&mut self) -> &str;
    fn open_with(&mut self, _obj: &Box<BaseObject>) -> &str;
}

// Take

pub trait Take {
    fn take(&self) -> &str;
}

// Go

pub trait Go {
    fn go_through(&self) -> &str;
}

// BaseObject

pub trait BaseObject: IntoBoxed<View> + IntoBoxed<Open> + IntoBoxed<Take> + IntoBoxed<Go> {
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

impl IntoBoxed<dyn Take> for Door {
    fn into_boxed(&self) -> Option<Box<&(dyn Take + 'static)>> {
        Option::None
    }

    fn into_boxed_mut(&mut self) -> Option<Box<&mut (dyn Take + 'static)>> {
        Option::None
    }
}

impl IntoBoxed<dyn Open> for Door {
    fn into_boxed(&self) -> Option<Box<&(dyn Open + 'static)>> {
        let w = self as &Open;
        Option::Some(Box::new(w))
    }

    fn into_boxed_mut(&mut self) -> Option<Box<&mut (dyn Open + 'static)>> {
        let p = self as &mut Open;
        Option::Some(Box::new(p))
    }
}

impl IntoBoxed<dyn View> for Door {
    fn into_boxed(&self) -> Option<Box<&(dyn View + 'static)>> {
        let w = self as &View;
        Option::Some(Box::new(w))
    }

    fn into_boxed_mut(&mut self) -> Option<Box<&mut (dyn View + 'static)>> {
        let p = self as &mut View;
        Option::Some(Box::new(p))
    }
}

impl IntoBoxed<dyn Go> for Door {
    fn into_boxed(&self) -> Option<Box<&(dyn Go + 'static)>> {
        let w = self as &Go;
        Option::Some(Box::new(w))
    }

    fn into_boxed_mut(&mut self) -> Option<Box<&mut (dyn Go + 'static)>> {
        let p = self as &mut Go;
        Option::Some(Box::new(p))
    }
}
pub trait BaseObject {

    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn view(&self) -> &str {
        self.description()
    }

    fn open(&self) -> &str {
        "Oops! Don't know how to open that"
    }

    fn take(&self) -> &str {
        "Oops! Cannot take that"
    }

}

pub struct Arrow {
    pub name: String,
    pub description: String
}

impl BaseObject for Arrow {

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

} 
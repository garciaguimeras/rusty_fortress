use super::env;

pub trait BaseObject {

    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn view(&self, environment: &env::Environment) -> &str {
        self.description()
    }

    fn open(&self, environment: &env::Environment) -> &str {
        "Oops! Don't know how to open that"
    }

    fn open_with(&self, environment: &env::Environment, obj: &Box<BaseObject>) -> &str {
        "Oops! Don't know how to open that"
    }

    fn take(&self, environment: &env::Environment) -> &str {
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
// Object
pub trait Name {
    fn name(&self) -> String;
}

// View
pub trait View {
    fn view(&mut self) -> String;
}

// Open
pub trait Open {
    fn open(&mut self) -> String;
}

// Open with
pub trait OpenWith {
    fn open_with(&mut self, key: &str) -> String;
}

// Take
pub trait Take {
    fn take(&mut self) -> String;
}

// Go throug
pub trait GoThrough {
    fn go_through(&mut self) -> String;
}
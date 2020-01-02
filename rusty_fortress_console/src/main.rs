extern crate rusty_fortress_lib;

use crate::rusty_fortress_lib::app::app;
mod console;

fn main() {
    let console = console::Console::new();
    app::run(console);
}

mod console;
mod obj;
mod app;

fn main() {
    let console = console::Console::new();
    app::app::run(&console);
}

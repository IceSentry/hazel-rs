use log::trace;
pub trait Application {
    fn run(&self);
}

pub fn hello() {
    println!("Welcome to hazel-rs");
}

pub struct Game {}

impl Application for Game {
    fn run(&self) {
        println!("running Game");
    }
}

pub fn create_app() -> Game {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    trace!("Initialized logging");
    Game {}
}

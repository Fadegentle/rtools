#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate log4rs;

mod app;
mod pre;

fn main() {
    pre::init();
    app::run();
}

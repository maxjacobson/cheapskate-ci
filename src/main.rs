#[macro_use]
extern crate clap;

mod cli;
use cli::App;

fn main() {
    App::run();
}

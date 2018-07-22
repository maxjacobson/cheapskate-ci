#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate psst;
extern crate reqwest;
extern crate toml;

mod cli;
mod config_file;
mod git;
mod github;
mod step;
mod step_runner;

use cli::App;

fn main() {
    env_logger::init();

    App::run();
}

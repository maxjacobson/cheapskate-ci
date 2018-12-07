mod cli;
mod config_file;
mod git;
mod github;
mod step;
mod step_runner;

use crate::cli::App;
use std::env;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "cheapskate_ci=info");
    }

    env_logger::init();

    App::run();
}

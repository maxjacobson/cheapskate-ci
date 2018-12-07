use crate::config_file::ConfigFile;
use crate::step::Runnable;

pub struct StepRunner;

impl StepRunner {
    pub fn run(config_file: &ConfigFile) {
        for step in config_file.steps() {
            step.run();
        }
    }
}

use config_file::ConfigFile;

pub struct StepRunner;

impl StepRunner {
    pub fn run() {
        for step in ConfigFile::new().steps() {
            step.run();
        }
    }
}

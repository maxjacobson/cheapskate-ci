use log::info;
use std::process;
use std::str;

pub trait Runnable {
    fn run(&self);
}

#[derive(Clone, Debug)]
pub struct Step {
    command: String,
}

impl Step {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

impl Runnable for Step {
    fn run(&self) {
        info!("OK, going to run step: {:?}", self.command);

        let mut parts = self.command.split_whitespace();
        let command = parts
            .nth(0)
            .unwrap_or_else(|| panic!("Invalid command: {}", self.command));

        let output = process::Command::new(command)
            .args(parts)
            .output()
            .unwrap_or_else(|_| panic!("Failed to run {}", self.command));

        if output.status.success() {
            info!("Command passed: {}", &self.command);
        } else {
            let stderr = str::from_utf8(&output.stderr)
                .unwrap_or_else(|_| panic!("Failed to parse stderr of command: {}", self.command));
            println!("{}", stderr);
            panic!("Command failed: {}", &self.command);
        }
    }
}

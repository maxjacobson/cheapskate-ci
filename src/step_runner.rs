use std::fs::File;
use std::io::prelude::*;
use std::process;
use toml;

use std::str;

#[derive(Clone, Debug)]
struct Step {
    command: String,
}

impl Step {
    fn new(command: String) -> Self {
        Self { command }
    }

    fn run(&self) {
        info!("OK, going to run step: {:?}", self.command);

        let mut parts = self.command.split_whitespace();
        let command = parts
            .nth(0)
            .expect(&format!("Invalid command: {}", self.command));

        let output = process::Command::new(&command)
            .args(parts)
            .output()
            .expect(&format!("Failed to run {}", self.command));

        if output.status.success() {
            info!("Command passed: {}", &self.command);
        } else {
            let stderr = str::from_utf8(&output.stderr).expect(&format!(
                "Failed to parse stderr of command: {}",
                self.command
            ));
            println!("{}", stderr);
            panic!("Command failed: {}", &self.command);
        }
    }
}

pub struct StepRunner;

impl StepRunner {
    pub fn run() {
        for step in Self::steps() {
            step.run();
        }
    }

    fn steps() -> Vec<Step> {
        let mut f = File::open("./cheapskate-ci.toml").expect("cheapskate-ci.toml not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let value = contents
            .parse::<toml::Value>()
            .expect("Could not parse file");

        let steps: &Vec<Step> = &value["ci"]["steps"]
            .as_array()
            .expect("steps wasn't an array")
            .iter()
            .map(|value| Step::new(value.as_str().expect("step wasn't a string").to_string()))
            .collect();

        (*steps).clone()
    }
}

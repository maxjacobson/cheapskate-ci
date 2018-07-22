use std::process;
use std::str;

#[derive(Clone, Debug)]
pub struct Step {
    command: String,
}

impl Step {
    pub fn new(command: String) -> Self {
        Self { command }
    }

    pub fn run(&self) {
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

use std::fs::File;
use std::io::prelude::*;
use step::Step;
use toml;

pub struct ConfigFile {
    value: toml::Value,
}

impl ConfigFile {
    pub fn new() -> Self {
        let mut f = File::open("./cheapskate-ci.toml").expect("cheapskate-ci.toml not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let value = contents
            .parse::<toml::Value>()
            .expect("Could not parse file");

        ConfigFile { value }
    }

    pub fn steps(&self) -> Vec<Step> {
        self.value["ci"]["steps"]
            .as_array()
            .expect("steps wasn't an array")
            .iter()
            .map(|value| Step::new(value.as_str().expect("step wasn't a string").to_string()))
            .collect()
    }

    pub fn repo_full_name(&self) -> String {
        self.value["github"]["repo"]
            .as_str()
            .expect("Could not look up repo")
            .to_string()
    }
}

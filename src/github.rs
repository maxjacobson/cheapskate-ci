use crate::config_file::ConfigFile;
use crate::git;
use log::{debug, info};
use psst;
use reqwest;
use std::collections::HashMap;
use std::process;
use std::str;

static CONTEXT: &'static str = "cheapskate-ci";

pub struct Status;

impl Status {
    pub async fn send_success(config_file: &ConfigFile) {
        let token = psst::new("cheapskate-ci")
            .expect("Could not initialize psst")
            .get("github_token")
            .expect("Could not read github_token");

        let user = Self::capture("whoami");
        let hostname = Self::capture("hostname");
        let description = format!("All steps passed locally on {}@{}", user, hostname);

        let mut payload = HashMap::new();
        payload.insert("state", "success");
        payload.insert("description", &description);
        payload.insert("context", CONTEXT);

        let repo_full_name = config_file.repo_full_name();
        let sha = git::get_latest_sha();

        let url = format!(
            "https://api.github.com/repos/{}/statuses/{}",
            repo_full_name, sha
        );

        debug!("Going to send status: {:?} to {}", payload, url);

        let client = reqwest::Client::builder()
            .build()
            .expect("Could not build client");

        let response = client
            .post(&url)
            .json(&payload)
            .bearer_auth(token)
            .send()
            .await
            .expect("Could not send request");

        info!("Response status: {:?}", response.status());
        debug!("Full response: {:#?}", response);
    }

    fn capture(command: &str) -> String {
        str::from_utf8(
            &process::Command::new(command)
                .output()
                .expect(&format!("Could not run {}", command))
                .stdout,
        )
        .expect(&format!("Could not convert `{}` to string", command))
        .trim()
        .to_string()
    }
}

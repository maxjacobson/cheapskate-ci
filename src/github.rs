use config_file::ConfigFile;
use git;
use psst;
use reqwest;
use std::collections::HashMap;
use std::process;
use std::str;

static CONTEXT: &'static str = "cheapskate-ci";

pub struct Status;

impl Status {
    pub fn send_success() {
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

        let repo_full_name = ConfigFile::new().repo_full_name();
        let sha = git::get_latest_sha();

        let url = format!(
            "https://api.github.com/repos/{}/statuses/{}",
            repo_full_name, sha
        );

        info!("Going to send status: {:?} to {}", payload, url);

        let mut headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::Authorization(format!("token {}", token)));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Could not build client");

        let response = client
            .post(&url)
            .json(&payload)
            .send()
            .expect("Could not send request");

        info!("Response: {:#?}", response);
    }

    fn capture(command: &str) -> String {
        str::from_utf8(
            &process::Command::new(command)
                .output()
                .expect(&format!("Could not run {}", command))
                .stdout,
        ).expect(&format!("Could not convert `{}` to string", command))
            .trim()
            .to_string()
    }
}

use config_file::ConfigFile;
use git;
use std::process;
use std::str;

static CONTEXT: &'static str = "cheapskate-ci";

#[derive(Debug)]
enum State {
    // Error,
    // Failure,
    // Pending,
    Success,
}

#[derive(Debug)]
struct StatusPayload {
    state: State,
    target_url: String,
    description: String,
    context: String,
}

pub struct Status;

impl Status {
    pub fn send_success() {
        let user = Self::capture("whoami");
        let hostname = Self::capture("hostname");

        let payload = StatusPayload {
            state: State::Success,
            target_url: "https://hardscrabble.net".to_string(),
            description: format!("All steps passed locally on {}@{}", user, hostname),
            context: CONTEXT.to_string(),
        };

        let repo_full_name = ConfigFile::new().repo_full_name();
        let sha = git::get_latest_sha();

        let url = format!(
            "https://api.github.com/repos/{}/statuses/{}",
            repo_full_name, sha
        );

        info!("Going to send status: {:?} to {}", payload, url);
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

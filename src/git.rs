use std::process;
use std::str;

pub fn get_latest_sha() -> String {
    let output = process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Could not run git rev-parse HEAD");

    if output.status.success() {
        str::from_utf8(&output.stdout)
            .expect("Could not read stdout of git rev-parse HEAD")
            .to_string()
    } else {
        panic!("git rev-parse HEAD exited non-zero");
    }
}

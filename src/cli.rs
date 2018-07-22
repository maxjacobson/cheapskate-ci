use clap::{App as ClapApp, AppSettings, SubCommand};
use step_runner::StepRunner;

pub struct App;

impl App {
    pub fn run() {
        let matches = ClapApp::new("Cheapskate CI")
            .version(crate_version!())
            .about("Run your CI locally")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("run").about("Run all of the steps"))
            .get_matches();

        if let Some(_matches) = matches.subcommand_matches("run") {
            StepRunner::run();
        } else {
            unreachable!()
        }
    }
}

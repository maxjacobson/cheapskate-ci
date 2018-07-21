use clap::{App as ClapApp, AppSettings, SubCommand};

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
            println!("OK, going to run your steps");
        } else {
            unreachable!()
        }
    }
}

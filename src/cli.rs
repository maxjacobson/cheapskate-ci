use clap::{App as ClapApp, AppSettings, Arg, SubCommand};
use github::Status;
use step_runner::StepRunner;

pub struct App;

impl App {
    pub fn run() {
        let matches = ClapApp::new("Cheapskate CI")
            .version(crate_version!())
            .about("Run your CI locally")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .subcommand(
                SubCommand::with_name("run")
                    .about("Run all of the steps")
                    .arg(
                        Arg::with_name("send status")
                            .short("s")
                            .long("status")
                            .help("Send status to GitHub if this arg is passed")
                            .takes_value(false),
                    ),
            )
            .get_matches();

        if let Some(matches) = matches.subcommand_matches("run") {
            StepRunner::run();

            if matches.is_present("send status") {
                Status::send_success();
            } else {
                debug!("Not going to send status");
            }
        } else {
            unreachable!()
        }
    }
}

use crate::config_file::ConfigFile;
use crate::github::Status;
use crate::step_runner::StepRunner;
use clap::{Parser, Subcommand};
use log::debug;

pub struct App;

#[derive(Parser)]
#[command(version, about, long_about = None, subcommand_required = true)]
struct ClapCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// Send status to GitHub if this arg is passed
        #[arg(short, long)]
        status: bool,
    },
}

impl App {
    pub async fn run() {
        let cli = ClapCli::parse();

        match &cli.command {
            Some(Commands::Run { status }) => {
                let config_file = ConfigFile::new();

                StepRunner::run(&config_file);

                if *status {
                    Status::send_success(&config_file).await;
                } else {
                    debug!("Not going to send status");
                }
            }
            None => {
                unreachable!();
            }
        }
    }
}

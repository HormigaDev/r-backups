use clap::{App, Arg, Command};
use database::client::DBClient;
use std::env;

mod database {
    pub mod backup;
    pub mod client;
    pub mod migration;
    pub mod restore;
    pub mod update;
}

mod commands;
mod config;
mod files;
mod handler;
mod vars;

#[tokio::main]
async fn main() {
    let file_path = env::current_dir()
        .expect("Unable to get current directory")
        .join(".r-backups");
    config::cli::load_config(&file_path.display().to_string());
    let db = DBClient::get_cli_connection().await;

    let mut matches = App::new("r-backups")
        .version("0.1.4")
        .author("HormigaDev <hormigadev7@gmail.com>")
        .about("Tool for managing databases and backups");

    for option in config::cli::get_options() {
        let (subcommand, about, args) = option;
        let mut subcmd = Command::new(subcommand);
        subcmd = subcmd.about(about);
        if args.len() > 0 {
            for arg in args {
                let (name, s, required, takes) = arg;
                subcmd = subcmd.arg(
                    Arg::new(name)
                        .short(s)
                        .long(name)
                        .takes_value(takes)
                        .required(required),
                );
            }
        }
        matches = matches.subcommand(subcmd);
    }

    let matches = matches.get_matches();
    handler::execute(db, matches).await;
}

use super::commands;
use super::commands::enums::Command;
use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(db: DBClient, matches: ArgMatches) {
    if let Some((name, sub_matches)) = matches.subcommand() {
        let command = Command::from_str(name);

        match command {
            Some(Command::Createdb) => {
                commands::createdb(sub_matches, &db).await;
            }
            Some(Command::List) => {
                commands::list(&db).await;
            }
            Some(Command::Rename) => {
                commands::rename(sub_matches, &db).await;
            }
            Some(Command::Drop) => {
                commands::drop(sub_matches, &db).await;
            }
            Some(Command::Count) => {
                commands::count(&db).await;
            }
            Some(Command::Backup) => {
                commands::backup(sub_matches, &db).await;
            }
            Some(Command::Restore) => {
                commands::restore(sub_matches, &db).await;
            }
            Some(Command::Init) => {
                commands::init(&db).await;
            }
            Some(Command::Migration) => {
                commands::migration(sub_matches, &db).await;
            }
            Some(Command::Update) => {
                commands::update(sub_matches).await;
            }
            _ => {
                println!("{}", "Unknown or unsupported command".red());
            }
        }
    } else {
        println!("{}", "Please enter the subcommand".red());
    }
}

use std::process;

use crate::database;
use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, db: &DBClient) {
    let db_name = sub_matches.value_of("database").unwrap_or_else(|| {
        eprintln!("{}", "Database name is required.".yellow());
        process::exit(1);
    });
    println!(
        "{} {}...",
        "Creating backup of the database".bright_blue(),
        db_name.green()
    );
    if let Err(e) = database::backup::create(&db, db_name).await {
        eprintln!("{}", format!("Error creating backup: {}", e).red());
        process::exit(1);
    }
}

use std::process;

use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, db: &DBClient) {
    let db_name = sub_matches.value_of("database").unwrap_or_else(|| {
        eprintln!("{}", "Database name is required.".red());
        process::exit(1);
    });
    let msg = format!("Dropping database {}...", db_name.green());
    println!("{}", msg.bright_blue());
    if let Err(e) = db.drop_database(db_name, false).await {
        eprintln!("{}", format!("Error dropping the database: {}", e).red());
        process::exit(1);
    }
}

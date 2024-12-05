use std::process;

use crate::database;
use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, mut db: &DBClient) {
    let db_name = sub_matches.value_of("database").unwrap();
    let file_path = sub_matches.value_of("file").unwrap();
    println!(
        "{} {} {} '{}'...",
        "Restoring the database".bright_blue(),
        db_name.green(),
        "from the file".bright_blue(),
        file_path.green()
    );
    if let Err(e) = database::restore::one(&mut db, db_name, file_path).await {
        eprintln!("{} {}", "Error restoring backup:".red(), e);
        process::exit(1);
    }
}

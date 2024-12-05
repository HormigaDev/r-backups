use std::process;

use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, db: &DBClient) {
    let db_name = sub_matches.value_of("name").unwrap_or_else(|| {
        eprintln!("{}", "Database name is required.".red());
        process::exit(1);
    });
    let sql_file_path = sub_matches.value_of("sql").unwrap_or_else(|| "");
    let deletedb = sub_matches.is_present("delete");
    println!(
        "{}",
        format!("Creating database {}...", db_name.green()).bright_blue()
    );
    if let Err(e) = db.create_database(db_name, sql_file_path, deletedb).await {
        eprintln!("{}", format!("Error creating the database: {}", e).red());
        process::exit(1);
    }
}

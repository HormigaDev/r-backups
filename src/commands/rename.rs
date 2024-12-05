use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, db: &DBClient) {
    let db_name = sub_matches.value_of("database").unwrap();
    let to_name = sub_matches.value_of("to").unwrap();
    println!(
        "{}",
        format!(
            "Renaming database '{}' to '{}'",
            db_name.green(),
            to_name.green()
        )
        .bright_blue()
    );
    db.rename_database(db_name, to_name).await;
}

use std::process;

use crate::database::client::DBClient;
use clap::ArgMatches;
use colored::*;
use postgres::types::ToSql;

pub async fn execute(sub_matches: &ArgMatches) {
    const SQL_ADD_CONFIG: &str = include_str!("../sql/add_config.sql");

    let key = sub_matches.value_of("key").unwrap_or_else(|| "");
    let value = sub_matches.value_of("value").unwrap_or_else(|| "");

    if key.is_empty() {
        eprintln!("{}", "The key is required.".red());
        process::exit(1);
    }
    if value.is_empty() {
        eprintln!("{}", "The value is required.".red());
        process::exit(1);
    }

    let clidb = DBClient::get_cli_connection().await;

    let params: [&(dyn ToSql + Sync); 2] = [&key, &value];
    match clidb.client.query(SQL_ADD_CONFIG, &params).await {
        Ok(_) => {
            println!(
                "{}",
                format!("Config '{}' saved sucessfully", key.green()).bright_blue()
            );
        }
        Err(e) => {
            eprintln!("{} {}", "Error saving config:".red(), e);
            process::exit(1);
        }
    }
}

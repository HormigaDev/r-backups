use clap::ArgMatches;
use colored::*;
use std::process;

use crate::database::{self, client::DBClient};

pub async fn execute(sub_matches: &ArgMatches) {
    let apply = sub_matches.is_present("apply");
    let rollback = sub_matches.is_present("rollback");
    let db_group = sub_matches.value_of("group").unwrap_or_else(|| "");
    let db_name = sub_matches.value_of("database").unwrap_or_else(|| "");

    if apply && rollback {
        eprintln!(
            "{}",
            format!(
                "The {} and {} flags cannot be specified together.",
                "--apply (-A)".yellow(),
                "--rollback (-R)".yellow()
            )
            .red()
        );
        process::exit(1);
    }
    if !apply && !rollback {
        eprintln!("{}", "No options matches. Please input any option".red());
        process::exit(1);
    }

    if db_group.is_empty() && db_name.is_empty() {
        eprintln!(
            "{}",
            "Please specify a group database (--group or -G) or a database name (--database or -d)"
                .red()
        );
        process::exit(1);
    }

    let clidb = DBClient::get_cli_connection().await;
    if apply {
        database::update::apply(db_group, db_name, &clidb).await;
    } else {
        database::update::rollback(db_group, db_name, &clidb).await;
    }
}

use std::process;

use crate::database::{self, client::DBClient};
use clap::ArgMatches;
use colored::*;

pub async fn execute(sub_matches: &ArgMatches, db: &DBClient) {
    let generate = sub_matches.is_present("generate");
    let dbgroup = sub_matches.value_of("group").unwrap_or_else(|| "");
    let name = sub_matches.value_of("name").unwrap_or_else(|| "");
    let id = sub_matches.value_of("id").unwrap_or_else(|| "");
    let up = sub_matches.is_present("up");
    let down = sub_matches.is_present("down");

    if !generate && dbgroup.is_empty() && name.is_empty() && id.is_empty() && !up && !down {
        eprintln!("{}", "No options matches. Please input any option".red());
        process::exit(1);
    }

    if generate && (!id.is_empty() || up || down) {
        eprintln!(
            "{}",
            "When using the --generate (-g) flag, no additional flags are allowed.".red()
        );
        process::exit(1);
    }

    if generate {
        if dbgroup.is_empty() {
            eprintln!(
                "{}",
                format!(
                    "Database group is required, use {} flag to provide a database group name",
                    "--group (-G)".yellow()
                )
                .red()
            );
            process::exit(1);
        }
        if name.is_empty() {
            eprintln!(
                "{}",
                format!(
                    "Migration name is required, use {} flag to provide a migration name",
                    "--name (-n)".yellow()
                )
                .red()
            );
            process::exit(1);
        }

        database::migration::generate(dbgroup, name).await;
    }

    if up && down {
        eprintln!(
            "{}",
            format!(
                "The {} and {} flags cannot be specified together.",
                "--up (-U)".yellow(),
                "--down (-D)".yellow()
            )
            .red()
        );
        process::exit(1);
    }

    if up && id.is_empty() || down && id.is_empty() {
        eprintln!(
            "{}",
            format!(
                "Migration id is required, use {} flag to provide a migration id",
                "--id (-i)".yellow()
            )
            .red()
        );
        process::exit(1);
    }

    if up && dbgroup.is_empty() || down && dbgroup.is_empty() {
        eprintln!(
            "{}",
            format!(
                "Database group is required, use {} flag to provide a database group name",
                "--group (-G)".yellow()
            )
            .red()
        );
        process::exit(1);
    }

    if up || down {
        let database_names = db.list_databases_with_prefix(dbgroup).await;

        let mut _option: &str = &String::new();

        if up {
            _option = "up";
        } else {
            _option = "down";
        }
        let mut updates: i64 = -1;

        for db_name in database_names {
            let groupdb = DBClient::get_db_connection(&db_name).await;
            database::migration::execute(_option, id.to_string(), &groupdb, true, &mut updates)
                .await;
        }
    }
}

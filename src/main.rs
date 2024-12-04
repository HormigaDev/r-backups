use clap::{App, Arg, Command};
use colored::*;
use database::client::DBClient;
use std::process;

mod database {
    pub mod backup;
    pub mod client;
    pub mod restore;
}

mod config;

#[tokio::main]
async fn main() {
    let mut db = match DBClient::connect().await {
        Ok(client) => client,
        Err(e) => {
            let err = format!("Error connecting to the database: {}", e);
            println!("{}", err.red());
            process::exit(1);
        }
    };

    let mut matches = App::new("r-backups")
        .version("0.1.0")
        .author("HormigaDev <hormigadev7@gmail.com>")
        .about("Tool for managing databases and backups");

    for option in config::cli::get_options() {
        let (subcommand, about, args) = option;
        let mut subcmd = Command::new(subcommand);
        subcmd = subcmd.about(about);
        if args.len() > 0 {
            for arg in args {
                let (name, s, takes, required) = arg;
                subcmd = subcmd.arg(
                    Arg::new(name)
                        .short(s)
                        .takes_value(takes)
                        .required(required),
                );
            }
        }
        matches = matches.subcommand(subcmd);
    }

    let matches = matches.get_matches();

    match matches.subcommand() {
        Some(("createdb", sub_matches)) => {
            let db_name = sub_matches.value_of("name").unwrap();
            let sql_file_path = sub_matches.value_of("sql").unwrap();
            let deletedb = sub_matches.is_present("delete");
            let msg = format!("Creating database {}...", db_name.green());
            println!("{}", msg.bright_blue());
            if let Err(e) = db.create_database(db_name, sql_file_path, deletedb).await {
                let err = format!("Error creating the database: {}", e);
                eprintln!("{}", err.red());
            }
        }
        Some(("list", _)) => match db.list_databases().await {
            Ok(databases) => {
                for dtb in databases {
                    println!("{}", dtb.green());
                }
            }
            Err(e) => {
                let err = format!("Error listing the databases: {}", e);
                eprintln!("{}", err.red())
            }
        },
        Some(("rename", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            let to_name = sub_matches.value_of("to").unwrap();
            let msg = format!(
                "Renaming database '{}' to '{}'",
                db_name.green(),
                to_name.green()
            );
            println!("{}", msg.bright_blue());
            if let Err(e) = db.rename_database(db_name, to_name).await {
                let err = format!("Error renaming the database: {}", e);
                eprintln!("{}", err.red());
            }
        }
        Some(("drop", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            let msg = format!("Dropping database {}...", db_name.green());
            println!("{}", msg.bright_blue());
            if let Err(e) = db.drop_database(db_name, false).await {
                let err = format!("Error dropping the database: {}", e);
                eprintln!("{}", err.red());
            }
        }
        Some(("count", _)) => {
            println!("Counting the databases...");
            match db.get_database_count().await {
                Ok(count) => {
                    println!(
                        "{} {}",
                        "Number of databases:".bright_blue(),
                        count.to_string().green()
                    );
                }
                Err(e) => {
                    let err = format!("Error counting the databases: {}", e);
                    eprintln!("{}", err.red());
                }
            }
        }
        Some(("create", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            println!(
                "{} {}...",
                "Creating backup of the database".bright_blue(),
                db_name.green()
            );
            database::backup::create(&db, db_name)
                .await
                .expect("Error creating backup");
        }
        Some(("restore", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            let file_path = sub_matches.value_of("file").unwrap();
            println!(
                "{} {} {} '{}'...",
                "Restoring the database".bright_blue(),
                db_name.green(),
                "from the file".bright_blue(),
                file_path.green()
            );
            database::restore::one(&mut db, db_name, file_path)
                .await
                .expect("Error restoring database");
        }
        _ => unreachable!(),
    }
}

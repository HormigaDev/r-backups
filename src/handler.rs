use crate::{commands, database::db::DBClient};
use clap::ArgMatches;
use colored::*;
use std::process::exit;

pub async fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("createdb", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap_or_else(|| {
                eprintln!("El nombre de la base de datos es necesario");
                exit(1);
            });
            let file_path = sub_matches.value_of("file").unwrap_or_else(|| "");

            commands::create_database(name, file_path).await;
        }
        Some(("generate-migration", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap_or_else(|| {
                eprintln!("El nombre de la migración es necesario");
                exit(1);
            });
            let group = sub_matches.value_of("group").unwrap_or_else(|| {
                eprintln!("El nombre del grupo es requerido");
                exit(1);
            });

            commands::generate_migration(name, group);
        }
        Some(("update", sub_matches)) => {
            let group = sub_matches.value_of("group").unwrap_or_else(|| {
                eprintln!("El nombre del grupo es requerido");
                exit(1);
            });

            commands::update(group).await;
        }
        Some(("backup", sub_matches)) => {
            let dbname = sub_matches.value_of("database").unwrap_or_else(|| {
                eprintln!("El nombre de la base de datos es necesario");
                exit(1);
            });
            let db = DBClient::from_db(dbname).await;
            db.backup().await;
        }
        Some(("restore", sub_matches)) => {
            let dbname = sub_matches.value_of("database").unwrap_or_else(|| {
                eprintln!("El nombre de la base de datos es necesario");
                exit(1);
            });
            let backup_file = sub_matches.value_of("file").unwrap_or_else(|| {
                eprintln!("La ubicación del archivo es necesaria");
                exit(1);
            });
            DBClient::restore(dbname, backup_file).await;
        }
        Some(("migration-up", sub_matches)) => {
            let dbname = sub_matches.value_of("database").unwrap_or_else(|| "");
            let groupdb = sub_matches.value_of("group").unwrap_or_else(|| "");
            let migration_id = sub_matches.value_of("migration").unwrap_or_else(|| {
                eprintln!("El identificador de la migración es necesario");
                exit(1);
            });

            commands::migration_up(migration_id, dbname, groupdb).await;
        }
        Some(("migration-down", sub_matches)) => {
            let dbname = sub_matches.value_of("database").unwrap_or_else(|| "");
            let groupdb = sub_matches.value_of("group").unwrap_or_else(|| "");
            let migration_id = sub_matches.value_of("migration").unwrap_or_else(|| {
                eprintln!("El identificador de la migración es necesario");
                exit(1);
            });

            commands::migration_down(migration_id, dbname, groupdb).await;
        }
        _ => {
            eprintln!("{}", "Comando desconocido o no soportado.".red());
            std::process::exit(1);
        }
    }
}

use colored::*;
use std::process::exit;

use crate::{
    database::db::DBClient,
    utils::{
        extract_migration_section, read_applied_changelog, read_migration_file,
        write_applied_changelog,
    },
};

pub async fn migration_up(migration_id: &str, dbname: &str, groupdb: &str) {
    if !dbname.is_empty() && !groupdb.is_empty() {
        eprintln!("{}", "Información conflictiva: No se puede informar el nombre de la base de datos junto con el grupo.".red());
        exit(1);
    }
    if dbname.is_empty() && groupdb.is_empty() {
        eprintln!(
            "{}",
            "Por favor informe el nombre de la base de datos o el grupo de base de datos.".yellow()
        );
        exit(1);
    }

    let migration_content = read_migration_file(migration_id);
    let up_sql = extract_migration_section(&migration_content, true);

    if !dbname.is_empty() {
        let mut applieds = read_applied_changelog(dbname);
        let db = DBClient::from_db(dbname).await;
        if applieds.contains(&migration_id.to_string()) {
            println!(
                "{}",
                format!(
                    "Migración no aplicada en la base de datos {}, omitiendo...",
                    dbname.bright_blue()
                )
                .yellow()
            );
        } else {
            db.client
                .batch_execute(&up_sql)
                .await
                .unwrap_or_else(|err| {
                    eprintln!(
                        "{}: {}",
                        format!(
                            "Ocurrió un error al intentar actualizar la base de datos {}",
                            dbname.yellow()
                        )
                        .red(),
                        err
                    );
                    exit(1);
                });
            applieds.push(migration_id.to_string());
            write_applied_changelog(dbname, applieds);
            println!("{}", "Base de datos actualizada".green());
        }
    }

    if !groupdb.is_empty() {
        let db = DBClient::from_db("").await;
        let databases = db.list_databases().await;
        for db_name in databases {
            if !db_name.starts_with(groupdb) {
                continue;
            }
            let mut applieds = read_applied_changelog(&db_name);
            let new_db = DBClient::from_db(&db_name).await;
            if applieds.contains(&migration_id.to_string()) {
                println!(
                    "{}",
                    format!(
                        "Migración ya aplicada en la base de datos {}, omitiendo...",
                        db_name.bright_blue()
                    )
                    .yellow()
                );
                continue;
            }
            if let Err(err) = new_db.client.batch_execute(&up_sql).await {
                eprintln!("{}: {}", format!("Ocurrió un error al intentar actualizar la base de datos {} con la migración {}", db_name.yellow(), migration_id.yellow()).red(), err);
                continue;
            }
            applieds.push(migration_id.to_string());
            write_applied_changelog(&db_name, applieds);
            println!(
                "{}",
                format!(
                    "Base de datos {} actualizada correctamente",
                    db_name.yellow()
                )
                .green()
            );
        }
    }
}

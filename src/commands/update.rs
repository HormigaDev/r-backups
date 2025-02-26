use crate::database::db::DBClient;
use crate::utils;
use crate::utils::{
    read_applied_changelog, read_changelog, read_migration_file, write_applied_changelog,
};
use colored::*;

pub async fn update(group: &str) {
    let db = DBClient::from_db("").await;
    let databases = db.list_databases().await;

    for dbname in databases {
        if !dbname.starts_with(group) {
            continue;
        }
        let new_db = DBClient::from_db(&dbname).await;
        apply(group, new_db, &dbname).await;
    }
}

async fn apply(group: &str, new_db: DBClient, dbname: &str) {
    let entries = read_changelog(group);
    let mut applieds = read_applied_changelog(dbname);

    for entry in &entries {
        if applieds.contains(&entry) {
            continue;
        } else {
            let migration_content = read_migration_file(&entry);
            let up_block = utils::extract_migration_section(&migration_content, true);

            if let Err(err) = new_db.client.batch_execute(&up_block).await {
                eprintln!(
                    "{}: {}",
                    format!(
                        "Error al actualizar la base de datos {} con la migración {}",
                        dbname.yellow(),
                        entry.yellow()
                    )
                    .red(),
                    err
                );
                continue;
            } else {
                println!("{}", dbname.green());
            }

            applieds.push(entry.to_string());
        }
    }

    write_applied_changelog(dbname, applieds);
}

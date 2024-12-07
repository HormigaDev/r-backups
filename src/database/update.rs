use crate::vars;
use std::process;

use super::client::DBClient;
use colored::*;

pub async fn apply(db_group: &str, db_name: &str, clidb: &DBClient) {
    let path = vars::get_chagelog_file_path();
    let changelog = super::migration::read_changelog(path);
    let mut updates: i64 = 0;
    if !db_group.is_empty() {
        let databases = clidb.list_databases_with_prefix(db_group).await;
        for dtb in databases {
            let db = DBClient::get_db_connection(&dtb).await;
            for entry in changelog.iter() {
                if entry.group == db_group {
                    super::migration::execute("up", entry.id.clone(), &db, false, &mut updates)
                        .await;
                }
            }
            println!(
                "{} {} '{}'\n",
                updates.to_string().yellow(),
                "Updated applied sucessfully for database".green(),
                db.get_name().yellow()
            );
            updates = 0;
        }
        println!(
            "{}",
            "Databases has been updated sucessfully.".bright_blue()
        );
    } else {
        let db = DBClient::get_db_connection(db_name).await;
        for entry in changelog.iter() {
            if db_name.starts_with(&entry.group) {
                super::migration::execute("up", entry.id.clone(), &db, false, &mut updates).await;
            }
        }
        println!(
            "{} {} '{}'\n",
            updates.to_string().yellow(),
            "Updated applied sucessfully for database".green(),
            db.get_name().yellow()
        );
    }
}

pub async fn rollback(db_group: &str, db_name: &str, clidb: &DBClient) {
    let path = vars::get_chagelog_file_path();
    let mut changelog = super::migration::read_changelog(path);
    let mut updates: i64 = 0;
    if !db_group.is_empty() {
        let databases = clidb.list_databases_with_prefix(db_group).await;
        changelog.retain(|x| x.group == db_group);
        changelog.sort();
        let entry = match changelog.last() {
            Some(entry) => entry,
            None => {
                eprintln!("{}", "Changelog is empty".yellow());
                process::exit(1);
            }
        };

        for dtb in databases {
            let db = DBClient::get_db_connection(&dtb).await;
            super::migration::execute("down", entry.id.clone(), &db, false, &mut updates).await;
            println!(
                "{} '{}'\n",
                "Last migration has been reverted sucessfully for database".green(),
                db.get_name().yellow()
            );
        }
    } else {
        let db = DBClient::get_db_connection(db_name).await;
        changelog.retain(|x| db_name.starts_with(&x.group));
        changelog.sort();
        let entry = match changelog.last() {
            Some(entry) => entry,
            None => {
                eprintln!("{}", "Changelog is empty".yellow());
                process::exit(1);
            }
        };
        super::migration::execute("down", entry.id.clone(), &db, false, &mut updates).await;
        println!(
            "{} '{}'\n",
            "Last migration has been reverted sucessfully for database".green(),
            db.get_name().yellow()
        );
    }
}

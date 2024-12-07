use super::client::DBClient;
use crate::vars;
use colored::*;
use postgres::types::ToSql;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct ChangelogEntry {
    pub id: String,
    pub group: String,
}

pub fn read_changelog<P: AsRef<Path>>(path: P) -> Vec<ChangelogEntry> {
    let file_content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "Error reading changelog content:".red(), e);
            process::exit(1);
        }
    };

    let changelog: Vec<ChangelogEntry> = match serde_json::from_str(&file_content) {
        Ok(changelog) => changelog,
        Err(e) => {
            eprintln!(
                "{} {}",
                "Error parsing chagelog to structure from string:".red(),
                e
            );
            process::exit(1);
        }
    };

    changelog
}

pub fn write_changelog<P: AsRef<Path>>(
    path: P,
    changelog: &Vec<ChangelogEntry>,
) -> Result<(), String> {
    // Convierte el changelog en un string JSON con formato legible.
    let json_content = serde_json::to_string_pretty(changelog)
        .map_err(|e| format!("{} {}", "Error serializing changelog to JSON:".red(), e))?;

    // Escribe el contenido JSON en el archivo especificado.
    fs::write(&path, json_content)
        .map_err(|e| format!("{} {}", "Error writing changelog to file:".red(), e))?;

    Ok(())
}

fn split_migration_sql(migration_sql: String) -> (String, String) {
    // Aquí dividimos el SQL en las partes 'up' y 'down'
    let mut up_sql = String::new();
    let mut down_sql = String::new();

    let mut current_block = "";

    for line in migration_sql.lines() {
        if line.trim().starts_with("-- up") {
            current_block = "up";
            continue;
        } else if line.trim().starts_with("-- down") {
            current_block = "down";
            continue;
        }

        if current_block == "up" {
            up_sql.push_str(line);
            up_sql.push_str("\n");
        } else if current_block == "down" {
            down_sql.push_str(line);
            down_sql.push_str("\n");
        }
    }

    (up_sql, down_sql)
}

fn get_migration_file(directory: &str, migration_id: &str) -> Option<PathBuf> {
    // Convertir el directorio y el identificador a un Path
    let dir_path = Path::new(directory);

    // Verificar si el directorio existe
    if !dir_path.is_dir() {
        eprintln!(
            "{}",
            format!("Directory '{}' not exists.", directory.yellow()).red()
        );
        process::exit(1);
    }

    // Crear el path del archivo concatenando el directorio, el ID de la migración y la extensión ".sql"
    let file_name = format!("{}.sql", migration_id);
    let file_path = dir_path.join(file_name);

    // Verificar si el archivo existe
    if file_path.exists() {
        Some(file_path)
    } else {
        eprintln!(
            "{}",
            format!("File '{}' not exists", file_path.display()).red()
        );
        None
    }
}

pub async fn execute(option: &str, id: String, db: &DBClient, log: bool, updates: &mut i64) {
    const UP_MIGRATION_SQL: &str = include_str!("../sql/up_migration.sql");
    const DOWN_MIGRATION_SQL: &str = include_str!("../sql/down_migration.sql");

    let migrations_dir = vars::get_migrations_dir();
    if migrations_dir.is_empty() {
        eprintln!("{}", "The migrations dir is not configured.".red());
        eprintln!(
            "{} {}",
            "To configure use:".bright_blue(),
            "r-backups config -k migrations -v path/to/migrations_dir/".yellow()
        );
        process::exit(1);
    }

    let migration_file_path = match get_migration_file(&migrations_dir, &id) {
        Some(option) => option,
        None => {
            eprintln!("{}", "File not found".red());
            process::exit(1);
        }
    };

    let content = match fs::read_to_string(migration_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "Error reading migration file:".red(), e);
            process::exit(1);
        }
    };

    let clidb = DBClient::get_cli_connection().await;
    let (up_sql, down_sql) = split_migration_sql(content);

    match option {
        "up" => {
            if clidb.exists_migration(&db.get_name(), &id).await {
                if log {
                    println!(
                        "{} '{}'\n",
                        "Migration already applied for database".yellow(),
                        db.get_name().green()
                    );
                }
                return;
            }
            if log {
                println!(
                    "{}",
                    format!("Updating database '{}'...", db.get_name().yellow()).bright_blue()
                );
            }
            match db.client.batch_execute(&up_sql).await {
                Ok(_) => {
                    let params: [&(dyn ToSql + Sync); 2] = [&db.get_name(), &id];
                    match clidb.client.query(UP_MIGRATION_SQL, &params).await {
                        Ok(_) => {
                            if log {
                                println!(
                                    "{} '{}'\n",
                                    "Migration applied sucessfully for database".green(),
                                    db.get_name().yellow()
                                );
                            }
                            *updates += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "{} '{}': {} {}",
                                "Error updating database with migration".red(),
                                id.yellow(),
                                up_sql.green(),
                                e
                            );
                            process::exit(1);
                        }
                    };
                }
                Err(e) => {
                    eprintln!(
                        "{} '{}': {} {}",
                        "Error updating database with migration".red(),
                        id.yellow(),
                        up_sql.green(),
                        e
                    );
                    process::exit(1);
                }
            }
        }
        "down" => {
            if !clidb.exists_migration(&db.get_name(), &id).await {
                if log {
                    println!(
                        "{} '{}'\n",
                        "Migration is not applied for database".yellow(),
                        db.get_name().green()
                    );
                }
                return;
            }
            if log {
                println!(
                    "{} '{}' {} '{}'",
                    "Rolling back migration".bright_blue(),
                    id.yellow(),
                    "for database".bright_blue(),
                    db.get_name().yellow()
                );
            }
            match db.client.batch_execute(&down_sql).await {
                Ok(_) => {
                    let params: [&(dyn ToSql + Sync); 2] = [&db.get_name(), &id];
                    match clidb.client.query(DOWN_MIGRATION_SQL, &params).await {
                        Ok(_) => {
                            if log {
                                println!(
                                    "{} '{}'\n",
                                    "Migration rollback sucessfully for database".green(),
                                    db.get_name().yellow()
                                );
                            }
                            *updates += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "{} '{}': {}",
                                "Error rolling back migration".red(),
                                id.yellow(),
                                e
                            );
                            process::exit(1);
                        }
                    };
                }
                Err(e) => {
                    eprintln!(
                        "{} '{}': {}",
                        "Error rolling back migration".red(),
                        id.yellow(),
                        e
                    );
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("{}", "Unsupported option for migration execute:".red());
            process::exit(1);
        }
    }
}

pub async fn generate(db_group: &str, name: &str) {
    let changelog_dir = vars::get_chagelog_file_path();
    let migrations_dir = vars::get_migrations_dir();
    if changelog_dir.is_empty() {
        eprintln!("{}", "Changelog dir is not configured".red());
        eprintln!(
            "{} {}",
            "To configure use".bright_blue(),
            "r-backups -k changelog -v path/to/changelogfile".yellow()
        );
        process::exit(1);
    }
    if migrations_dir.is_empty() {
        eprintln!("{}", "The migrations dir is not configured.".red());
        eprintln!(
            "{} {}",
            "To configure use:".bright_blue(),
            "r-backups config -k migrations -v path/to/migrations_dir/".yellow()
        );
        process::exit(1);
    }

    let chagelog_dir = changelog_dir.replace("\\", "/");

    let path = Path::new(&chagelog_dir);

    let path = match fs::canonicalize(path) {
        Ok(absolute_path) => absolute_path,
        Err(_) => path.to_path_buf(),
    };

    let mut data = read_changelog(&path);

    let mut last_id = String::new();

    for entry in data.iter().clone() {
        if entry.group == db_group {
            let entryspl: Vec<&str> = entry.id.splitn(2, "_").collect();
            if entry.id > last_id {
                last_id = entryspl[0].to_string();
            }
        }
    }

    if last_id.is_empty() {
        last_id = String::from("00000");
    }

    let new_id = get_id(last_id);
    let new_entry = ChangelogEntry {
        id: format!("{}_{}", new_id, name),
        group: db_group.to_string(),
    };

    data.push(new_entry.clone());

    match write_changelog(path, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", "Error writting changelog file:".red(), e);
            process::exit(1);
        }
    };

    make_migration_template(&migrations_dir, &new_entry.id);
}

fn make_migration_template(dir: &str, name: &str) {
    const MIGRATION_TEMPLATE: &str = include_str!("../sql/migration_template.sql");
    let migration_file = format!("{}/{}.sql", dir, name);

    if !Path::new(&migration_file).exists() {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "Error creating directories for migrations".red(),
                    e
                );
                process::exit(1);
            }
        };
    }

    match fs::write(migration_file, MIGRATION_TEMPLATE) {
        Ok(_) => {
            println!(
                "{}",
                format!("Generated migration '{}'", name.green()).bright_blue()
            );
        }
        Err(e) => {
            eprintln!("{} {}", "Error writting new migration:".red(), e);
            process::exit(1);
        }
    };
}

fn get_id(last_id: String) -> String {
    let length = last_id.len();

    let mut num: u32 = match last_id.parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!(
                "{} {} {} {}",
                "Error parsing id to number, the id".red(),
                last_id.yellow(),
                "is invalid".red(),
                e
            );
            process::exit(1);
        }
    };

    num += 1;

    let new_id = format!("{:0width$}", num, width = length);

    new_id
}

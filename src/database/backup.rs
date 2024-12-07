use super::super::vars;
use super::client::DBClient;
use colored::*;
use std::process::{self, Command};
use tokio_postgres::Error;

pub async fn create(db: &DBClient, db_name: &str) -> Result<(), Error> {
    let backups_dir = vars::get_backups_dir();
    if backups_dir.is_empty() {
        eprintln!("{}", "Backup directory is not defined".red());
        eprintln!(
            "{}",
            "Use r-backups config --key (-k) backups --value (-v) path/to/backups_dir/".yellow()
        );
        process::exit(1);
    }
    let db_password = vars::get_password();
    let backup_file = format!("{}{}.backup.sql", backups_dir, db_name);
    let output = match Command::new("pg_dump")
        .env("PGPASSWORD", db_password)
        .arg("-h")
        .arg(&db.host())
        .arg("-p")
        .arg(vars::get_port())
        .arg("-U")
        .arg(&db.user())
        .arg("-F")
        .arg("c")
        .arg("-f")
        .arg(&backup_file)
        .arg(db_name)
        .output()
    {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} {}", "Error executing pg_dump:".red(), e);
            process::exit(1);
        }
    };

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Error creating the backup:".red(),
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    }

    println!(
        "Backup of the database '{}' created successfully",
        db_name.green()
    );
    Ok(())
}

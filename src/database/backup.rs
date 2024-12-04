use super::client::DBClient;
use colored::*;
use std::process::Command;
use tokio_postgres::Error;

pub async fn create(db: &DBClient, db_name: &str) -> Result<(), Error> {
    let db_password = DBClient::get_env("password");
    let backup_file = format!("backups/{}.backup.sql", db_name);
    let output = Command::new("pg_dump")
        .env("PGPASSWORD", db_password)
        .arg("-h")
        .arg(&db.host())
        .arg("-U")
        .arg(&db.user())
        .arg("-F")
        .arg("c")
        .arg("-f")
        .arg(&backup_file)
        .arg(db_name)
        .output()
        .expect(&format!("{}", "Error executing pg_dump".red()));

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Error creating the backup:".red(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!(
        "Backup of the database '{}' created successfully",
        db_name.green()
    );
    Ok(())
}

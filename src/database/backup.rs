use super::client::DBClient;
use std::env;
use std::process::Command;
use tokio_postgres::Error;

pub async fn create(db: &DBClient, db_name: &str) -> Result<(), Error> {
    let db_password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
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
        .expect("Error al ejecutar pg_dump");

    if !output.status.success() {
        eprintln!(
            "Error al crear el backup: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!(
        "Backup de la base de datos '{}' creado exitosamente",
        db_name
    );
    Ok(())
}

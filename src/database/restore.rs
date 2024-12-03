use super::client::DBClient;
use colored::*;
use std::process::Command;
use tokio_postgres::Error;

pub async fn one(db: &DBClient, db_name: &str, backup_file: &str) -> Result<(), Error> {
    let db_password = std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

    db.drop_database(db_name, true).await?;
    db.create_database(db_name).await?;

    // Ejecutar el comando pg_restore
    let output = Command::new("pg_restore")
        .env("PGPASSWORD", db_password) // Usar la contraseña desde el entorno
        .arg("-h")
        .arg(&db.host()) // Dirección del host
        .arg("-U")
        .arg(&db.user()) // Usuario de la base de datos
        .arg("-d")
        .arg(db_name) // Base de datos de destino
        .arg(backup_file) // Archivo de backup binario
        .output()
        .expect(&format!("{}", "Error executing pg_restore".red()));

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Error restoring the backup:".red(),
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!(
            "Backup of the database '{}' restored successfully",
            db_name.green()
        );
    }

    Ok(())
}

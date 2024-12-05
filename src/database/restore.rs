use super::client::DBClient;
use colored::*;
use std::{
    path::Path,
    process::{self, Command},
};
use tokio_postgres::Error;

pub async fn one(db: &DBClient, db_name: &str, backup_file: &str) -> Result<(), Error> {
    let db_password = DBClient::get_env("password");

    let file = Path::new(backup_file);

    if !file.exists() {
        eprintln!(
            "{}",
            format!("File '{}' not exists", backup_file.yellow()).red()
        );
        process::exit(1);
    }

    db.drop_database(db_name, true).await?;
    db.create_database(db_name, "", true).await?;

    // Ejecutar el comando pg_restore
    let output = match Command::new("pg_restore")
        .env("PGPASSWORD", db_password) // Usar la contraseña desde el entorno
        .arg("-h")
        .arg(&db.host())
        .arg("-p")
        .arg(DBClient::get_env("dbport")) // Dirección del host
        .arg("-U")
        .arg(&db.user()) // Usuario de la base de datos
        .arg("-d")
        .arg(db_name) // Base de datos de destino
        .arg(backup_file) // Archivo de backup binario
        .output()
    {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} {}", "Error executing pg_restore".red(), e);
            process::exit(1);
        }
    };

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Error restoring the backup:".red(),
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    } else {
        println!(
            "Backup of the database '{}' restored successfully",
            db_name.green()
        );
    }

    Ok(())
}

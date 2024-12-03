use super::client::DBClient;
use std::process::Command;
use tokio_postgres::Error;

pub async fn one(db: &DBClient, db_name: &str, backup_file: &str) -> Result<(), Error> {
    let db_password = std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

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
        .expect("Error al ejecutar pg_restore");

    if !output.status.success() {
        eprintln!(
            "Error al restaurar el backup: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!(
            "Backup de la base de datos '{}' restaurado exitosamente",
            db_name
        );
    }

    Ok(())
}

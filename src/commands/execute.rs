use crate::database::db::DBClient;
use colored::*;
use std::{fs, process::exit};

pub async fn execute(dbname: &str, file_path: &str) {
    let db = DBClient::from_db(dbname).await;
    let sql_content = fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!(
            "{}: {}",
            format!("Error al leer el archivo {}", file_path.yellow()).red(),
            error
        );
        exit(1);
    });

    if let Err(error) = db.client.batch_execute(&sql_content).await {
        eprintln!(
            "{}: {}",
            "Error al ejecutar script en la base de datos".red(),
            error
        );
        exit(1);
    }

    println!(
        "{}",
        format!(
            "Script {} ejecutado con éxito en la base de datos {}",
            file_path.yellow(),
            dbname.yellow()
        )
        .green()
    );
}

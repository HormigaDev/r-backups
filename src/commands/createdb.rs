use crate::database::db::DBClient;
use colored::*;
use std::{fs, process::exit};

pub async fn create_database(name: &str, file_path: &str) {
    let db = DBClient::from_db("").await;

    let query = format!("CREATE DATABASE {}", name);
    db.client.execute(&query, &[]).await.unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("Error al crear la base de datos {}", name.yellow()).red()
        );
        exit(1);
    });

    println!(
        "{}",
        format!("Base de datos {} creada con éxito", name.yellow()).green()
    );

    if !file_path.is_empty() {
        let sql_content = fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!("Error al leer el archivo {}", file_path.yellow()).red()
            );
            exit(1);
        });

        let new_db = DBClient::from_db(name).await;

        new_db
            .client
            .batch_execute(&sql_content)
            .await
            .unwrap_or_else(|_| {
                eprintln!("{}", "Error al ejecutar script en la base de datos".red());
                exit(1);
            });

        println!(
            "{}",
            format!(
                "Script {} ejecutado con éxito en la base de datos {}",
                file_path.yellow(),
                name.yellow()
            )
            .green()
        );
    }
}

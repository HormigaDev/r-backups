use crate::database::db::DBClient;
use colored::*;

pub async fn list() {
    let db = DBClient::from_db("").await;
    let databases = db.list_databases().await;

    println!(
        "{}",
        "Estas son todas las bases de datos presentes:".bright_purple()
    );
    for dbname in databases {
        println!("[::] {}", dbname.bright_blue());
    }
}

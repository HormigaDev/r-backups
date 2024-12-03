use bbel_common::terminal::input;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub struct DBClient {
    client: Client,
}

impl DBClient {
    pub async fn connect() -> Result<DBClient, Error> {
        dotenv().ok();

        let db_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());

        let connection_string = format!(
            "host={} user={} password={} dbname={}",
            db_host, db_user, db_password, db_name
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        tokio::spawn(connection);

        Ok(DBClient { client })
    }

    pub async fn get_database_count(&self) -> Result<i64, Error> {
        let rows = self
            .client
            .query("SELECT COUNT(*) FROM pg_databases;", &[])
            .await?;
        let count: i64 = rows[0].get(0);

        Ok(count)
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, Error> {
        let rows = self
            .client
            .query("SELECT datname FROM pg_databases;", &[])
            .await?;
        let databases: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

        Ok(databases)
    }

    pub async fn create_database(&self, db_name: &str) -> Result<(), Error> {
        self.client
            .execute(&format!("CREATE DATABASE {}", db_name), &[])
            .await?;

        Ok(())
    }

    pub async fn rename_database(&self, old_name: &str, new_name: &str) -> Result<(), Error> {
        self.client
            .execute(
                &format!("ALTER TABLE {} RENAME TO {}", old_name, new_name),
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn drop_database(&self, db_name: &str) -> Result<(), Error> {
        let input = input(format!("¿Estás seguro que deseas eliminar la base de datos '{}'? Escribe 'yes' para confirmar:", db_name).as_str());
        let input = input.trim();

        if input == "yes" {
            self.client
                .execute(&format!("DROP DATABASE IF EXITS {}", db_name), &[])
                .await?;
            println!(
                "La base de datos '{}' ha sido eliminada correctamente.",
                db_name
            );
        } else {
            println!("Operación cancelada. No se ha eliminado la base de datos.");
        }

        Ok(())
    }
}

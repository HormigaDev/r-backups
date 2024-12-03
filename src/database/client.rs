use bbel_common::terminal::input;
use colored::*;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub struct DBClient {
    pub client: Client,
    host: String,
    user: String,
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

        Ok(DBClient {
            client,
            host: db_host,
            user: db_user,
        })
    }

    pub async fn get_database_count(&self) -> Result<i64, Error> {
        let rows = self
            .client
            .query("SELECT COUNT(*) FROM pg_catalog.pg_database;", &[])
            .await?;
        let count: i64 = rows[0].get(0);

        Ok(count)
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, Error> {
        let rows = self
            .client
            .query("SELECT datname FROM pg_catalog.pg_database;", &[])
            .await?;
        let databases: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

        Ok(databases)
    }

    pub async fn create_database(&self, db_name: &str) -> Result<(), Error> {
        self.client
            .execute(&format!("CREATE DATABASE {}", db_name), &[])
            .await?;

        println!("Database '{}' created successfully!", db_name.green());
        Ok(())
    }

    pub async fn rename_database(&self, old_name: &str, new_name: &str) -> Result<(), Error> {
        self.client
            .execute(
                &format!("ALTER TABLE {} RENAME TO {}", old_name, new_name),
                &[],
            )
            .await?;

        println!("{}", "Operation completed successfully!".green());
        Ok(())
    }

    pub async fn drop_database(&self, db_name: &str, confirm: bool) -> Result<(), Error> {
        let input = match confirm {
            false => input(
                &format!(
                    "Are you sure you want to drop the database '{}'? Type 'yes' to confirm:",
                    db_name.yellow()
                )
                .as_str(),
            ),
            true => "yes".to_string(),
        };
        let input = input.trim();

        if input == "yes" {
            let _ = self
            .client
            .execute(
                &format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid();", db_name),
                &[]
            ).await?;

            self.client
                .execute(&format!("DROP DATABASE IF EXISTS {}", db_name), &[])
                .await?;
            println!(
                "The database '{}' has been dropped successfully.",
                db_name.green()
            );
        } else {
            println!(
                "{}",
                "Operation canceled. The database was not dropped.".yellow()
            );
        }

        Ok(())
    }

    pub fn host(&self) -> String {
        return self.host.clone();
    }

    pub fn user(&self) -> String {
        return self.user.clone();
    }
}

use bbel_common::terminal::input;
use colored::*;
use dotenv::dotenv;
use postgres::types::ToSql;
use std::{env, process};
use tokio_postgres::{Client, Error, NoTls};

pub struct DBClient {
    pub client: Client,
    host: String,
    user: String,
    name: String,
}

impl DBClient {
    pub fn get_env(key: &str) -> String {
        dotenv().ok();
        let db_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());
        let cli_db_password =
            env::var("CLI_DB_PASSWORD").unwrap_or_else(|_| "db_password".to_string());
        let cli_user = env::var("CLI_USER").unwrap_or_else(|_| "user".to_string());
        let db_port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());

        match key {
            "host" => db_host,
            "user" => db_user,
            "password" => db_password,
            "rootdbname" => db_name,
            "clidbpassword" => cli_db_password,
            "cliuser" => cli_user,
            "dbport" => db_port,
            _ => String::new(),
        }
    }

    fn get_connection_string(db_name: &str) -> String {
        let db_host = DBClient::get_env("host");
        let db_user = DBClient::get_env("user");
        let db_password = DBClient::get_env("password");
        let db_port = DBClient::get_env("dbport");

        format!(
            "host={} user={} password={} dbname={} port={}",
            db_host, db_user, db_password, db_name, db_port
        )
    }

    pub async fn get_db_connection(db_name: &str) -> DBClient {
        let db_host = DBClient::get_env("host");
        let db_user = DBClient::get_env("user");

        let connection_string = DBClient::get_connection_string(db_name);
        let (client, connection) = match tokio_postgres::connect(&connection_string, NoTls).await {
            Ok(value) => value,
            Err(e) => {
                eprintln!(
                    "{} '{}' {}",
                    "Error getting the connection for database".red(),
                    db_name.yellow(),
                    e
                );
                process::exit(1);
            }
        };

        tokio::spawn(connection);

        DBClient {
            client,
            host: db_host,
            user: db_user,
            name: db_name.to_string(),
        }
    }

    fn get_cli_connection_string(db_name: &str) -> String {
        let db_host = DBClient::get_env("host");
        let db_user = DBClient::get_env("cliuser");
        let db_password = DBClient::get_env("clidbpassword");
        let db_port = DBClient::get_env("dbport");

        format!(
            "host={} user={} password={} dbname={} port={}",
            db_host, db_user, db_password, db_name, db_port
        )
    }

    pub async fn get_cli_connection() -> DBClient {
        let db_host = DBClient::get_env("host");
        let db_user = DBClient::get_env("cliuser");
        let db_name = DBClient::get_env("rootdbname");

        let connection_string = DBClient::get_cli_connection_string(&db_name);

        let (client, connection) = match tokio_postgres::connect(&connection_string, NoTls).await {
            Ok(value) => value,
            Err(e) => {
                eprintln!(
                    "{} '{}': {}",
                    "Error connecting to the CLI database".red(),
                    db_name.yellow(),
                    e
                );
                process::exit(1);
            }
        };

        tokio::spawn(connection);

        DBClient {
            client,
            host: db_host,
            user: db_user,
            name: db_name,
        }
    }

    pub async fn connect() -> DBClient {
        let db_host = DBClient::get_env("host");
        let db_user = DBClient::get_env("user");
        let db_name = DBClient::get_env("rootdbname");

        let connection_string = DBClient::get_connection_string(&db_name);

        let (client, connection) = match tokio_postgres::connect(&connection_string, NoTls).await {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error connecting to the database:".red(), e);
                process::exit(1);
            }
        };
        tokio::spawn(connection);

        DBClient {
            client,
            host: db_host,
            user: db_user,
            name: db_name,
        }
    }

    pub async fn get_database_count(&self) -> i64 {
        let rows = match self
            .client
            .query("SELECT COUNT(*) FROM pg_catalog.pg_database;", &[])
            .await
        {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error getting databases count:".red(), e);
                process::exit(1);
            }
        };

        rows[0].get(0)
    }

    pub async fn list_databases(&self) -> Vec<String> {
        let rows = match self
            .client
            .query("SELECT datname FROM pg_catalog.pg_database;", &[])
            .await
        {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error listing databases:".red(), e);
                process::exit(1);
            }
        };

        rows.iter().map(|row| row.get(0)).collect()
    }

    async fn database_exists(&self, db_name: &str) -> bool {
        let connection_string = DBClient::get_connection_string("bbel_postgres");
        let (client, connection) = match tokio_postgres::connect(&connection_string, NoTls).await {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error connecting to the database:".red(), e);
                process::exit(1);
            }
        };
        tokio::spawn(connection);

        // Ejecutar la consulta para verificar si la base de datos existe
        let rows = match client
            .query(
                "SELECT 1 FROM pg_catalog.pg_database WHERE datname = $1;",
                &[&db_name],
            )
            .await
        {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error checking if the database exists:".red(), e);
                process::exit(1);
            }
        };

        return !rows.is_empty();
    }

    pub async fn create_database(
        &self,
        db_name: &str,
        sql_file_path: &str,
        deletedb: bool,
    ) -> Result<(), Error> {
        let exists = self.database_exists(db_name).await;

        if exists && deletedb {
            match self.drop_database(db_name, true).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{} {}", "Error deleting database before create:".red(), e);
                    process::exit(1);
                }
            };
        }

        match self
            .client
            .execute(&format!("CREATE DATABASE {}", db_name), &[])
            .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{} {}", "Error creating the database:".red(), e);
                process::exit(1);
            }
        };

        if !sql_file_path.is_empty() {
            let clientdb = DBClient::get_db_connection(db_name).await;
            match std::fs::read_to_string(sql_file_path) {
                Ok(sql) => match clientdb.client.execute(&sql, &[]).await {
                    Ok(_) => {
                        println!(
                            "SQL file from '{}' executed sucessfully on database '{}'.",
                            sql_file_path.green(),
                            db_name.green()
                        )
                    }
                    Err(e) => {
                        eprintln!("{}", format!("Error executing SQL file: {}", e).red())
                    }
                },
                Err(e) => {
                    eprintln!("{}", format!("Error reading SQL file: {}", e).red());
                }
            }
        }

        println!("Database '{}' created successfully!", db_name.green());
        Ok(())
    }

    pub async fn list_databases_with_prefix(&self, prefix: &str) -> Vec<String> {
        let query = "SELECT datname FROM pg_catalog.pg_database WHERE datname LIKE $1;";
        let rows = match self.client.query(query, &[&format!("{}%", prefix)]).await {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{} {}", "Error listing databases with prefix:".red(), e);
                process::exit(1);
            }
        };

        rows.iter().map(|row| row.get(0)).collect()
    }

    pub async fn rename_database(&self, old_name: &str, new_name: &str) {
        // Verificar si la base de datos antigua existe
        let exists = self.database_exists(old_name).await;
        if !exists {
            eprintln!(
                "{} '{}' {}",
                "Error: The database to rename doesn't exist.".red(),
                old_name.yellow(),
                "Check the database name.".yellow()
            );
            process::exit(1);
        }

        // Asegurarse de que el nuevo nombre no esté en uso
        let exists_new = self.database_exists(new_name).await;
        if exists_new {
            eprintln!(
                "{} '{}' {}",
                "Error: The new database name is already in use.".red(),
                new_name.yellow(),
                "Choose a different name.".yellow()
            );
            process::exit(1);
        }

        // Renombrar la base de datos
        match self
            .client
            .execute(
                &format!("ALTER DATABASE {} RENAME TO {}", old_name, new_name),
                &[],
            )
            .await
        {
            Ok(_) => {
                println!(
                    "Database '{}' renamed successfully to '{}'.",
                    old_name.green(),
                    new_name.green()
                );
            }
            Err(e) => {
                eprintln!("{} {}", "Error renaming the database:".red(), e);
                process::exit(1);
            }
        }
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
            match self
            .client
            .execute(
                &format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid();", db_name),
                &[]
            ).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{} {}", "Error disconnecting users before drop database:", e);
                    process::exit(1);
                }
            };

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

    pub async fn get_config(config_key: &str) -> String {
        const SQL_GET_CONFIG: &str = include_str!("../sql/get_config.sql");

        let clidb = DBClient::get_cli_connection().await;

        let params: [&(dyn ToSql + Sync); 1] = [&config_key];

        let row = match clidb.client.query_opt(SQL_GET_CONFIG, &params).await {
            Ok(Some(row)) => row,
            Ok(None) => {
                eprintln!(
                    "{} {}",
                    "Configuration key not found:".red(),
                    config_key.yellow()
                );
                return "".to_string();
            }
            Err(e) => {
                eprintln!("{} {}", "Error getting configuration:".red(), e);
                process::exit(1);
            }
        };

        row.get("config_value")
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub async fn exists_migration(&self, db_name: &str, db_version: &str) -> bool {
        let query = "SELECT 1 FROM migrations WHERE db_name = $1 AND db_version = $2 LIMIT 1;";

        let params: [&(dyn ToSql + Sync); 2] = [&db_name, &db_version];

        match self.client.query_opt(query, &params).await {
            Ok(Some(_)) => true, // Existe la migración
            Ok(None) => false,   // No existe la migración
            Err(e) => {
                eprintln!("{} {}", "Error checking migration existence:".red(), e);
                process::exit(1);
            }
        }
    }
}

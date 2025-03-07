use crate::{config::Config, utils::get_rbackups_dir};
use chrono::Utc;
use colored::*;
use std::process::{exit, Command};
use tokio_postgres::{Client, NoTls, Row};

pub struct DBClient {
    pub client: Client,
    dbname: String,
}

fn get_str_connection(dbname: &str) -> String {
    let config = Config::app_config();
    let mut database_name = dbname.to_string();
    if database_name.is_empty() {
        database_name = config.database_name;
    }
    format!(
        "host={} port={} user={} password={} dbname={}",
        config.host, config.port, config.user, config.password, database_name
    )
}

impl DBClient {
    pub async fn from_db(db: &str) -> Self {
        let conn_str = get_str_connection(db);

        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
            .await
            .unwrap_or_else(|error| {
                eprintln!(
                    "{}: {}",
                    "Error al conectar a la base de datos".red(),
                    error
                );
                exit(1);
            });

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("{}: {}", "Fallo en la conexión".red(), err);
            }
        });

        DBClient {
            client,
            dbname: db.to_string(),
        }
    }

    pub async fn list_databases(&self) -> Vec<String> {
        let query = "SELECT datname FROM pg_database WHERE datistemplate = false";

        let rows = self.client.query(query, &[]).await.unwrap_or_else(|error| {
            eprintln!(
                "{}: {}",
                "Error al obtener la lista de bases de datos".red(),
                error
            );
            exit(1);
        });

        rows.iter()
            .map(|row: &Row| row.get::<_, String>(0))
            .collect()
    }

    pub async fn backup(&self) {
        let config = Config::app_config();
        let rbackups_dir = get_rbackups_dir();
        let backups_dir = &config.backups_dir.replace("$APP", &rbackups_dir);

        let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let backup_filename = format!(
            "{}/{}_{}_backup.sql.gz",
            backups_dir, self.dbname, timestamp
        );

        let output = Command::new("pg_dump")
            .arg("-h")
            .arg(&config.host)
            .arg("-p")
            .arg(config.port.to_string())
            .arg("-U")
            .arg(&config.user)
            .arg("-d")
            .arg(&self.dbname)
            .arg("-F")
            .arg("c")
            .arg("-Z")
            .arg("9")
            .arg("-f")
            .arg(&backup_filename)
            .env("PGPASSWORD", &config.password)
            .output()
            .unwrap_or_else(|error| {
                eprintln!("{}: {}", "Error al realizar el backup".red(), error);
                exit(1);
            });

        if !output.status.success() {
            eprintln!(
                "{}: {}",
                "Ocurrió un error al generar el backup".red(),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!(
            "{}",
            format!(
                "Backup exitoso de la base de datos {}. Guardado en {}",
                &self.dbname.yellow(),
                backup_filename.yellow()
            )
            .green()
        );
    }

    pub async fn restore(dbname: &str, backup_file: &str) {
        let config = Config::app_config();

        let db = DBClient::from_db("").await;

        let terminate_query = format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid();", &dbname);
        db.client
            .batch_execute(&terminate_query)
            .await
            .unwrap_or_else(|err| {
                eprintln!(
                    "{}: {}",
                    "Ocurrió un error al intentar terminar las conexiones de la base de datos"
                        .red(),
                    err
                );
                exit(1);
            });

        let query = format!("DROP DATABASE IF EXISTS {}", dbname);
        db.client.batch_execute(&query).await.unwrap_or_else(|err| {
            eprintln!("{}: {}", "Error al eliminar la base de datos".red(), err);
            exit(1);
        });

        let output = Command::new("psql")
            .arg("-h")
            .arg(&config.host)
            .arg("-p")
            .arg(config.port.to_string())
            .arg("-U")
            .arg(&config.user)
            .arg("-c")
            .arg(&format!("CREATE DATABASE {}", &dbname))
            .env("PGPASSWORD", &config.password)
            .output()
            .unwrap_or_else(|err| {
                eprintln!("{}: {}", "Error al crear la base de datos".red(), err);
                exit(1);
            });

        if !output.status.success() {
            eprintln!(
                "{} {}",
                "Error al crear la base de datos".red(),
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }

        let output = Command::new("pg_restore")
            .arg("-h")
            .arg(&config.host)
            .arg("-p")
            .arg(config.port.to_string())
            .arg("-U")
            .arg(&config.user)
            .arg("-d")
            .arg(&dbname)
            .arg("--clean")
            .arg("--if-exists")
            .arg("--no-owner")
            .arg(backup_file)
            .env("PGPASSWORD", &config.password)
            .output()
            .unwrap_or_else(|err| {
                eprintln!("{}: {}", "Error al restaurar la base de datos".red(), err);
                exit(1);
            });

        if !output.status.success() {
            eprintln!(
                "{}: {}",
                "Ocurrió un error al restaurar la base de datos".red(),
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }

        println!(
            "{}",
            format!(
                "Base de datos {} restaurada exitosamente desde {}",
                &dbname.yellow(),
                backup_file.yellow()
            )
            .green()
        );
    }
}

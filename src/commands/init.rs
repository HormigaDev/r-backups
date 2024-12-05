use crate::database::client::DBClient;
use colored::*;
use std::process;

pub async fn execute(db: &DBClient) {
    const INIT_SQL: &str = include_str!("../sql/init.sql");

    let check_table_query = "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'configurations');";
    let mut table_config_exists: bool = true;

    match db.client.query_opt(check_table_query, &[]).await {
        Ok(Some(row)) => {
            let table_exists: Option<bool> = row.get(0);
            if table_exists.is_none() {
                table_config_exists = false;
            }
        }
        Ok(None) => {
            table_config_exists = false;
        }
        Err(e) => {
            eprintln!("{} {}", "Error checking table existence:".red(), e);
            process::exit(1);
        }
    }

    let query =
        "SELECT config_value FROM configurations WHERE config_key = 'init_database' LIMIT 1";

    if table_config_exists {
        match db.client.query_opt(query, &[]).await {
            Ok(Some(row)) => {
                let config_value: String = row.get("config_value");
                if config_value == "applied" {
                    println!("{}", "Database already initialized.".yellow());
                    process::exit(1);
                }
            }
            Ok(None) => {}
            Err(_) => {}
        }
    }

    let query = INIT_SQL
        .replace("$password", &DBClient::get_env("clidbpassword"))
        .replace("$cli_user", &DBClient::get_env("cliuser"));

    match db.client.batch_execute(&query).await {
        Ok(_) => {
            println!("{}", "Database initialized sucessfully.".green());
        }
        Err(e) => {
            eprintln!("{} {}", "Error executing SQL script:".red(), e);
            process::exit(1);
        }
    }
}

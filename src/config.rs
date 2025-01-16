use colored::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::sync::Mutex;

pub mod cli {
    use super::*;

    // Variable global que almacenará la configuración cargada desde el archivo
    lazy_static! {
        static ref CONFIG: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    pub fn get_options() -> Vec<(
        &'static str,
        &'static str,
        Vec<(&'static str, char, bool, bool)>,
    )> {
        const REQUIRED: bool = true;
        const OPTIONAL: bool = false;
        const TAKES: bool = true;
        const NOTAKES: bool = false;

        vec![
            (
                "createdb",
                "Create a new database",
                vec![
                    ("name", 'n', REQUIRED, TAKES),
                    ("sql", 's', OPTIONAL, TAKES),
                    ("delete", 'D', OPTIONAL, NOTAKES),
                ],
            ),
            ("list", "List all databases", vec![]),
            (
                "rename",
                "Rename a database",
                vec![
                    ("database", 'd', REQUIRED, TAKES),
                    ("to", 't', REQUIRED, TAKES),
                ],
            ),
            (
                "drop",
                "Drop a database",
                vec![("database", 'd', REQUIRED, TAKES)],
            ),
            ("count", "Count the number of databases", vec![]),
            (
                "backup",
                "Create a backup of the specified database",
                vec![("database", 'd', REQUIRED, TAKES)],
            ),
            (
                "restore",
                "Restore a database from a backup file",
                vec![
                    ("database", 'd', REQUIRED, TAKES),
                    ("file", 'f', REQUIRED, TAKES),
                ],
            ),
            (
                "init",
                "This command starts the CLI with its default settings.",
                vec![],
            ),
            (
                "migration",
                "This command executes, reverts, or generates a migration.",
                vec![
                    ("generate", 'g', OPTIONAL, NOTAKES),
                    ("id", 'i', OPTIONAL, TAKES),
                    ("up", 'U', OPTIONAL, NOTAKES),
                    ("down", 'D', OPTIONAL, NOTAKES),
                    ("group", 'G', OPTIONAL, TAKES),
                    ("name", 'n', OPTIONAL, TAKES),
                ],
            ),
            (
                "update",
                "This command applies all pending migrations, bringing the databases up-to-date.",
                vec![
                    ("apply", 'A', OPTIONAL, NOTAKES),
                    ("group", 'G', OPTIONAL, TAKES),
                    ("rollback", 'R', OPTIONAL, NOTAKES),
                    ("database", 'd', OPTIONAL, TAKES),
                ],
            ),
        ]
    }

    pub fn load_config(file_path: &str) {
        let mut config_map = HashMap::new();

        let file = File::open(file_path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!("Failed to open config file: {}", file_path.yellow()).red()
            );
            exit(1);
        });

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                eprintln!(
                    "{}",
                    format!("Invalid line in config file: {}", line.yellow()).red()
                );
                exit(1);
            }
        }

        // Guardar la configuración en la variable global
        let mut config = CONFIG.lock().unwrap();
        *config = config_map;
    }

    pub fn get_value(key: &str) -> String {
        let config = CONFIG.lock().unwrap();
        config
            .get(key)
            .unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    format!("Missing required key: {}", key.yellow()).red()
                );
                exit(1);
            })
            .to_string()
    }
}

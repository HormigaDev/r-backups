use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::exit;

pub mod cli {
    pub fn get_options() -> Vec<(
        &'static str,
        &'static str,
        Vec<(&'static str, char, bool, bool)>,
    )> {
        const REQUIRED: bool = true;
        const OPTIONAL: bool = false;
        const TAKES: bool = true;
        //const NOTAKES: bool = false;

        vec![
            (
                "createdb",
                "Permite crear una nueva base de datos.",
                vec![
                    ("name", 'n', REQUIRED, TAKES),
                    ("file", 'f', OPTIONAL, TAKES),
                ],
            ),
            (
                "generate-migration",
                "Genera una nueva migración de la base de datos especificada.",
                vec![
                    ("name", 'n', REQUIRED, TAKES),
                    ("group", 'g', REQUIRED, TAKES),
                ],
            ),
            (
                "update",
                "Actualiza la base de datos, o el grupo de bases de datos seleccionado.",
                vec![("group", 'n', REQUIRED, TAKES)],
            ),
            (
                "backup",
                "Genera un backup de la base de datos especificada.",
                vec![("database", 'd', REQUIRED, TAKES)],
            ),
            (
                "restore",
                "Restaura una base de datos a partir de un backup.",
                vec![
                    ("database", 'd', REQUIRED, TAKES),
                    ("file", 'f', REQUIRED, TAKES),
                ],
            ),
            (
                "migration-up",
                "Ejecuta una migración especifica en una base de datos determinada",
                vec![
                    ("database", 'd', OPTIONAL, TAKES),
                    ("group", 'g', OPTIONAL, TAKES),
                    ("migration", 'm', REQUIRED, TAKES),
                ],
            ),
            (
                "migration-down",
                "Deshace una migración especifica en una base de datos determinada",
                vec![
                    ("database", 'd', OPTIONAL, TAKES),
                    ("group", 'g', OPTIONAL, TAKES),
                    ("migration", 'm', REQUIRED, TAKES),
                ],
            ),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database_name: String,
    pub port: u16,
    pub backups_dir: String,
    pub migrations_dir: String,
    pub changelogs_dir: String,
}

impl Config {
    pub fn app_config() -> Self {
        let path = "r-backups.json";
        if !Path::new(path).exists() {
            eprintln!(
                "{}",
                format!(
                    "ERROR: No se encontró el archivo de configuración {}",
                    path.yellow()
                )
                .red()
            );
            exit(1);
        }

        let file_content = fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!(
                    "ERROR: No se pudo leer el archivo de configuración {}",
                    path.yellow()
                )
                .red()
            );
            exit(1);
        });

        serde_json::from_str(&file_content).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!(
                    "ERROR: Formato inválido en el archivo de configuración {}",
                    path.yellow()
                )
                .red()
            );
            exit(1);
        })
    }
}

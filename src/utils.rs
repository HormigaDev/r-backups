use colored::*;
use std::env;
use std::path::Path;
use std::{fs, process::exit};

use crate::config::Config;

pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| {
        eprintln!("No se pudo obtener el directorio HOME.");
        exit(1);
    })
}

pub fn get_rbackups_dir() -> String {
    let home_dir = get_home_dir();
    format!("{}/.r-backups", home_dir)
}

pub fn create_r_backups_folder() {
    let folder_path = get_rbackups_dir();
    let path = Path::new(&folder_path);

    if !path.exists() {
        fs::create_dir_all(path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!("Error al crear la carpeta {}", folder_path.yellow()).red()
            );
            exit(1);
        });

        println!(
            "{}",
            format!("Carpeta {} creada correctamente", folder_path.yellow()).green()
        );
    }
}

pub fn extract_migration_section(content: &str, up: bool) -> String {
    let section_start = if up { "-- up" } else { "-- down" };
    let section_end = if up { "-- down" } else { "-- up" };

    if let Some(start_index) = content.find(section_start) {
        let start_pos = start_index + section_start.len();

        if let Some(end_index) = content[start_pos..].find(section_end) {
            let end_pos = start_pos + end_index;
            return content[start_pos..end_pos].trim().to_string();
        }

        return content[start_pos..].trim().to_string();
    }

    eprintln!(
        "{}",
        format!(
            "Error: No se encontró la sección '{}' en el archivo.",
            section_start
        )
        .red()
    );
    exit(1);
}

pub fn read_applied_changelog(name: &str) -> Vec<String> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("{}", "Error al obtener el directorio HOME.".red());
        exit(1);
    });

    let changelog_dir = format!("{}/.r-backups/changelogs", home_dir);
    let changelog_path = format!("{}/{}", changelog_dir, name);

    let path = Path::new(&changelog_path);

    if !path.exists() {
        if let Err(err) = fs::create_dir_all(&changelog_dir) {
            eprintln!(
                "{}: {}",
                format!("Error al crear el directorio {}", changelog_dir.yellow()).red(),
                err
            );
            exit(1);
        }

        let empty_content = "[]";
        if let Err(err) = fs::write(path, empty_content) {
            eprintln!(
                "{}: {}",
                format!("Error al crear el archivo {}", changelog_path.yellow()).red(),
                err
            );
            exit(1);
        }

        return Vec::new();
    }

    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("Error al leer el archivo {}", changelog_path.yellow()).red()
        );
        exit(1);
    });

    serde_json::from_str::<Vec<String>>(&content).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!(
                "El archivo {} está en un formato inválido, debe ser un array de strings",
                changelog_path.yellow()
            )
            .red()
        );
        exit(1);
    })
}

pub fn write_applied_changelog(name: &str, content: Vec<String>) {
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("{}", "Error al obtener el directorio HOME.".red());
        exit(1);
    });

    let changelog_dir = format!("{}/.r-backups/changelogs", home_dir);
    let changelog_path = format!("{}/{}", changelog_dir, name);

    let path = Path::new(&changelog_path);

    if !path.exists() {
        if let Err(err) = fs::create_dir_all(&changelog_dir) {
            eprintln!(
                "{}: {}",
                format!("Error al crear el directorio {}", changelog_dir.yellow()).red(),
                err
            );
            exit(1);
        }
    }

    let content_json = serde_json::to_string(&content).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!(
                "Error al serializar el contenido para el archivo {}",
                changelog_path.yellow()
            )
            .red()
        );
        exit(1);
    });

    if let Err(err) = fs::write(path, content_json) {
        eprintln!(
            "{}: {}",
            format!(
                "Error al escribir en el archivo {}",
                changelog_path.yellow()
            )
            .red(),
            err
        );
        exit(1);
    }
}

pub fn read_changelog(name: &str) -> Vec<String> {
    let config = Config::app_config();
    let log_path = format!("{}/{}_change.log", config.changelogs_dir, name);

    let path = Path::new(&log_path);
    if !path.exists() {
        eprintln!(
            "{}",
            format!("No se encontró el archivo {}", log_path.yellow()).red()
        );
        exit(1);
    }

    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("Error al leer el chagelog {}", log_path.yellow()).red()
        );
        exit(1);
    });

    serde_json::from_str::<Vec<String>>(&content).unwrap_or_else(|_| {
        eprintln!("{}", "Error al convertir el changelog".red());
        exit(1);
    })
}

pub fn write_changelog(name: &str, content: Vec<String>) {
    let config = Config::app_config();
    let log_path = format!("{}/{}_change.log", config.changelogs_dir, name);

    let path = Path::new(&log_path);

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            if let Err(err) = fs::create_dir_all(parent_dir) {
                eprintln!(
                    "{}: {}",
                    format!(
                        "Error al crear el directorio {}",
                        parent_dir.to_string_lossy().yellow()
                    )
                    .red(),
                    err
                );
                exit(1);
            }
        }
    }

    let content_json = serde_json::to_string(&content).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!(
                "Error al serializar el contenido para {}",
                log_path.yellow()
            )
            .red()
        );
        exit(1);
    });

    if let Err(err) = fs::write(path, content_json) {
        eprintln!(
            "{}: {}",
            format!("Error al escribir en el archivo {}", log_path.yellow()).red(),
            err
        );
        exit(1);
    }
}

pub fn read_migration_file(name: &str) -> String {
    let config = Config::app_config();
    let migrations_dir = &config.migrations_dir;

    let full_path = format!("{}/{}.sql", migrations_dir, name);
    let path = Path::new(&full_path);

    if !path.exists() {
        eprintln!(
            "{}",
            format!(
                "No se encontró el archivo de migración {}",
                full_path.yellow()
            )
            .red()
        );
        exit(1);
    }

    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("Error al leer el archivo {}", full_path.yellow()).red()
        );
        exit(1);
    })
}

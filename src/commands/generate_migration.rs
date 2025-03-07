use crate::{
    config::Config,
    utils::{read_changelog, write_changelog},
};
use colored::*;
use std::{fs, io::Write, path::Path, process::exit};

const TEMPLATE: &str = include_str!("../templates/migration-template.sql");

pub fn generate_migration(name: &str, group: &str) {
    let config = Config::app_config();
    let base_path = config.migrations_dir;
    let migration_id = format!("{}/{}", base_path, name);
    let mut migrations = read_changelog(group);
    if migrations.contains(&migration_id) {
        eprintln!("{}", "Ya existe una migración con ese identificador".red());
        exit(1);
    }
    migrations.push(migration_id);

    let full_path = format!("{}/{}.sql", base_path, name);
    let path = Path::new(&full_path);
    let parent_dir = path.parent().unwrap();

    fs::create_dir_all(parent_dir).unwrap_or_else(|error| {
        eprintln!(
            "{}: {}",
            format!(
                "Error al crear la carpeta {}",
                parent_dir.to_string_lossy().yellow()
            )
            .red(),
            error
        );
        exit(1);
    });

    let mut file = fs::File::create(path).unwrap_or_else(|error| {
        eprintln!(
            "{}: {}",
            format!("Error al crear el archivo {}", full_path.yellow()).red(),
            error
        );
        exit(1);
    });

    file.write_all(TEMPLATE.as_bytes()).unwrap_or_else(|error| {
        eprintln!(
            "{}: {}",
            format!("Error al escribir en el archivo {}", full_path.yellow()).red(),
            error
        );
        exit(1);
    });

    write_changelog(group, migrations);
    println!(
        "{}",
        format!("Migración {} generada con éxito.", full_path.yellow()).green()
    );
}

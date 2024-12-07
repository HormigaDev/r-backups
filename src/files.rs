use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs, process};

pub fn create_dir_if_not_exists(dir_path: &str) {
    let path = Path::new(dir_path);
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Directory '{}' created sucessfully.", dir_path.yellow()).green()
                );
            }
            Err(e) => {
                eprintln!(
                    "{} '{}': {}",
                    "Error creating directory".red(),
                    dir_path.yellow(),
                    e
                );
                process::exit(1);
            }
        };
    }
}

pub fn create_chagelog_file(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        let mut file = match OpenOptions::new().write(true).create(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!(
                    "{} '{}': {}",
                    "Error creating and opening chagelog file".red(),
                    file_path.yellow(),
                    e
                );
                process::exit(1);
            }
        };
        if let Err(e) = writeln!(file, "[]") {
            eprintln!("{} {}", "Error writing chagelog file".red(), e);
            process::exit(1);
        } else {
            println!(
                "{}",
                format!(
                    "Changelog file ('{}') created sucessfully.",
                    file_path.yellow()
                )
                .green()
            );
        }
    }
}

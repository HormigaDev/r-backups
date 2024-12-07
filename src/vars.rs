use std::env;

pub fn get_host() -> String {
    env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string())
}

pub fn get_user() -> String {
    env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string())
}

pub fn get_password() -> String {
    env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string())
}

pub fn get_dbroot_name() -> String {
    env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string())
}

pub fn get_port() -> String {
    env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string())
}

pub fn get_migrations_dir() -> String {
    env::var("MIGRATIONS_DIR").unwrap_or_else(|_| "./migrations/".to_string())
}

pub fn get_backups_dir() -> String {
    env::var("BACKUPS_DIR").unwrap_or_else(|_| "./backups/".to_string())
}

pub fn get_chagelog_file_path() -> String {
    env::var("CHANGELOG_FILE_PATH").unwrap_or_else(|_| "./changelog".to_string())
}

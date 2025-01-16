use crate::config::cli::get_value;

pub fn get_host() -> String {
    get_value("DATABASE_HOST")
}

pub fn get_user() -> String {
    get_value("DATABASE_USER")
}

pub fn get_password() -> String {
    get_value("DATABASE_PASSWORD")
}

pub fn get_dbroot_name() -> String {
    get_value("DATABASE_NAME")
}

pub fn get_port() -> String {
    get_value("DATABASE_PORT")
}

pub fn get_migrations_dir() -> String {
    get_value("MIGRATIONS_DIR")
}

pub fn get_backups_dir() -> String {
    get_value("BACKUPS_DIR")
}

pub fn get_chagelog_file_path() -> String {
    get_value("CHANGELOG_FILE_PATH")
}

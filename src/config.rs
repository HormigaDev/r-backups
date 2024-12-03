pub mod cli {
    pub fn get_options() -> [(
        &'static str,
        &'static str,
        Vec<(&'static str, char, bool, bool)>,
    ); 7] {
        let options: [(&str, &str, Vec<(&str, char, bool, bool)>); 7] = [
            (
                "createdb",
                "Create a new database",
                vec![("name", 'n', true, true)],
            ),
            ("list", "List all databases", vec![]),
            (
                "rename",
                "Rename a database",
                vec![("database", 'd', true, true), ("to", 't', true, true)],
            ),
            (
                "drop",
                "Drop a database",
                vec![("database", 'd', true, true)],
            ),
            ("count", "Count the number of databases", vec![]),
            (
                "backup",
                "Create a backup of the specified database",
                vec![("database", 'd', true, true)],
            ),
            (
                "restore",
                "Restore a database from a backup file",
                vec![("database", 'd', true, true), ("file", 'f', true, true)],
            ),
        ];
        return options;
    }
}

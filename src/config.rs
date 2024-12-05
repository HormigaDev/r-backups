pub mod cli {
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
                "config",
                "This command sets the CLI configurations.",
                vec![
                    ("key", 'k', OPTIONAL, TAKES),
                    ("value", 'v', OPTIONAL, TAKES),
                ],
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
}

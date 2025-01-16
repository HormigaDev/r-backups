# R-BACKUPS

This project is designed to manage centralized databases. The CLI performs operations such as backups, restores, creating databases from a `.sql` file, and more.

This project is designed for environments where `postgresql` is used.

## Docs language versions (Only Github)

-   [Spanish](docs/spanish.md)
-   [Portuguese](docs/portuguese.md)

## Table of Contents

-   [Installation](#installation)
-   [Usage](#usage)
-   [Configuration](#configuration)
-   [Commands](#commands)
    -   [Backup](#backup)
    -   [Restore](#restore)
    -   [Migration](#migration)
    -   [Update](#update)
    -   [Createdb](#createdb)
    -   [List](#list)
    -   [Count](#count)
    -   [Rename](#rename)
-   [Options](#options)
-   [Examples](#examples)
-   [Contribution](#contribution)
-   [License](#license)

## Installation

Instructions to install the CLI on different operating systems.

### Install via Cargo

```bash
cargo install r-backups
```

### Install from Source Code

1. Clone this repository:

```bash
git clone https://github.com/HormigaDev/r-backups.git
cd r-backups
```

2. Build and run:

```bash
cargo build --release
```

## Usage

Basic explanation of how to use the CLI.

```bash
r-backups [options] [arguments]
```

> <> means required and [] means optional

## Configuration

For the CLI to work correctly, you must have the following environment variables configured in a `.r-backups` file in the directory where you will use the CLI or in the system's environment variables.

```ini
DATABASE_HOST=<host>
DATABASE_USER=<user>
DATABASE_PASSWORD=<password>
DATABASE_NAME=<database name>
DATABASE_PORT=<port>
BACKUPS_DIR=path/to/backups_directory/
MIGRATIONS_DIR=path/to/migrations_directory/
CHANGELOG_FILE_PATH=path/to/chagelog
```

### Initial configuration to use the CLI correctly

```bash
r-backups init
```

Note:

> The changelog file conventionally has no extension, and its content must initially be an empty array or an array of objects in the following format:

```json
[
    {
        "id": "migration_id",
        "group": "database_prefix e.g., (company_)"
    }
]
```

## Commands

---

### `backup`

This command creates a backup of a specific database.

#### Usage:

```bash
r-backups backup --database <database_name>
```

For this command to work correctly, the `backups` configuration must be set.  
See [Configuration](#configuration).

---

### `restore`

This command restores a backup from a specified file.

#### Usage:

```bash
r-backups restore --database <database_name> --file path/to/backup_file.backup.sql
```

---

### `migration`

This command is used to apply or revert a specific migration for a group of databases.  
You can also use this command to generate a new migration.

#### Usage:

> Generating a new migration:

```bash
r-backups migration --generate --group <group_name> --name <migration_name>
```

When generating a migration, the CLI uses a unique identifier for each migration, consisting of a 5-digit numeric prefix.  
For example, the CLI could generate a migration named `00001_create_table_users.sql`.

The migration body contains the following:

```sql
-- up
your SQL code here;

-- down
your rollback SQL code here;
```

It is IMPORTANT not to delete the `-- up` or `-- down` comments so the CLI can identify them correctly.

> Updating a group of databases:

```bash
r-backups migration --up --id <migration_id> --group <group_name>
```

> Reverting a specific migration:

```bash
r-backups migration --down --id <migration_id> --group <group_name>
```

Note:  
For this command to work correctly, the `changelog` and `migrations` configurations must be set.  
See [Configuration](#configuration).

---

### `update`

This command applies all pending migrations to a group of databases or a specific database.  
It can also rollback the last migration for a group of databases or a specific database.

#### Usage:

> Updating a group of databases:

```bash
r-backups update --apply --group <group_name>
```

> Updating a specific database:

```bash
r-backups update --apply --database <database_name>
```

> Rolling back the last migration for a group of databases:

```bash
r-backups update --rollback --group <group_name>
```

> Rolling back the last migration for a specific database:

```bash
r-backups update --rollback --database <database_name>
```

---

### `createdb`

This command creates a database with a specified name.

#### Usage:

```bash
r-backups createdb --name <database_name> --sql [path/to/file.sql]
```

---

### `drop`

This command drops a database and requires confirmation before deletion.

#### Usage:

```bash
r-backups drop --database <database_name>
```

---

### `list`

This command lists all existing databases.

#### Usage:

```bash
r-backups list
```

---

### `count`

This command returns the number of databases present, including templates.

#### Usage:

```bash
r-backups count
```

---

### `rename`

This command renames a database to a specified new name.

#### Usage:

```bash
r-backups rename --database <old_name> --to <new_name>
```

---

## Options

| Option            | Description                                |
| ----------------- | ------------------------------------------ |
| `-h`, `--help`    | Displays the command's help information.   |
| `-v`, `--version` | Displays the current CLI version.          |
| `init`            | Initializes the CLI with default settings. |

## Examples

### Create a database

```bash
r-backups createdb --name example_1
```

## Contribution

Contributions are welcome! If you'd like to contribute, follow these steps:

1. Fork the repository.
2. Create a branch for your contribution (`git checkout -b my-feature`).
3. Make your changes and commit them.
4. Submit a pull request detailing what you've done.

Make sure to follow Rust style guides and include unit tests whenever possible.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

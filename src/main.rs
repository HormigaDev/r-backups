mod database {
    pub mod backup;
    pub mod client;
    pub mod restore;
}

use std::process;

use clap::{App, Arg, Command};
use database::client::DBClient;

#[tokio::main]
async fn main() {
    let mut db = match DBClient::connect().await {
        Ok(client) => client,
        Err(e) => {
            println!("Error al conectar a la base de datos: {}", e);
            process::exit(1);
        }
    };

    let yes = true;
    let matches = App::new("r-backups")
        .version("0.1.0")
        .author("HormigaDev <hormigadev7@gmail.com>")
        .about("Herramienta para gestionar bases de datos y respaldos")
        .subcommand(
            Command::new("createdb")
                .about("Crea una nueva base de datos")
                .arg(
                    Arg::new("database")
                        .short('d')
                        .long("database")
                        .takes_value(yes)
                        .required(yes),
                ),
        )
        .subcommand(Command::new("list").about("Lista todas las bases de datos"))
        .subcommand(
            Command::new("rename")
                .arg(
                    Arg::new("database")
                        .short('d')
                        .long("database")
                        .takes_value(yes)
                        .required(yes),
                )
                .arg(
                    Arg::new("to")
                        .short('t')
                        .long("to")
                        .takes_value(yes)
                        .required(yes),
                ),
        )
        .subcommand(
            Command::new("drop").about("Elimina una base de datos").arg(
                Arg::new("database")
                    .short('d')
                    .long("database")
                    .takes_value(yes)
                    .required(yes),
            ),
        )
        .subcommand(Command::new("count").about("Cuenta el número de bases de datos"))
        .subcommand(
            Command::new("create")
                .about("Crea un backup de la base de datos especificada")
                .arg(
                    Arg::new("database")
                        .short('d')
                        .long("database")
                        .takes_value(yes)
                        .required(yes),
                ),
        )
        .subcommand(
            Command::new("restore")
                .about("Restaura una base de datos desde un archivo de backup")
                .arg(
                    Arg::new("database")
                        .short('d')
                        .long("database")
                        .takes_value(yes)
                        .required(yes),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .takes_value(yes)
                        .required(yes),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("createdb", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            println!("Creando base de datos {}...", db_name);
            if let Err(e) = db.create_database(db_name).await {
                eprintln!("Error al crear la base de datos: {}", e);
            }
        }
        Some(("list", _)) => match db.list_databases().await {
            Ok(databases) => {
                for dtb in databases {
                    println!("{}", dtb);
                }
            }
            Err(e) => eprintln!("Error al listar las bases de datos: {}", e),
        },
        Some(("rename", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            let to_name = sub_matches.value_of("to").unwrap();
            println!("Renombrando base de datos '{}' a '{}'", db_name, to_name);
            if let Err(e) = db.rename_database(db_name, to_name).await {
                eprintln!("Error al renombrar la base de datos: {}", e);
            }
        }
        Some(("drop", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            println!("Eliminando base de datos {}...", db_name);
            if let Err(e) = db.drop_database(db_name, false).await {
                eprintln!("Error al eliminar la base de datos: {}", e);
            }
        }
        Some(("count", _)) => {
            println!("Contando las bases de datos...");
            match db.get_database_count().await {
                Ok(count) => println!("Número de bases de datos: {}", count),
                Err(e) => eprintln!("Erro al contar las bases de datos: {}", e),
            }
        }
        Some(("create", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            println!("Creando backup de la base de datos {}...", db_name);
            database::backup::create(&db, db_name)
                .await
                .expect("Error al crear backup");
        }
        Some(("restore", sub_matches)) => {
            let db_name = sub_matches.value_of("database").unwrap();
            let file_path = sub_matches.value_of("file").unwrap();
            println!(
                "Restaurando la base de datos '{}' desde el archivo '{}'...",
                db_name, file_path
            );
            database::restore::one(&mut db, db_name, file_path)
                .await
                .expect("Error al restaurar base de datos");
        }
        _ => unreachable!(),
    }
}

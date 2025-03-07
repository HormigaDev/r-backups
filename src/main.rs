use clap::{App, Arg, Command};
use dotenv::dotenv;

mod commands;
mod config;
mod database;
mod handler;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut matches = App::new("r-backups")
        .version("0.2.4")
        .author("HormigaDev <hormigadev7@gmail.com>")
        .about("Interfaz de línea de comandos para gestionar bases de datos y migraciones.");

    for option in config::cli::get_options() {
        let (subcommand, about, args) = option;
        let mut subcmd = Command::new(subcommand);
        subcmd = subcmd.about(about);
        if args.len() > 0 {
            for arg in args {
                let (name, s, required, takes) = arg;
                subcmd = subcmd.arg(
                    Arg::new(name)
                        .short(s)
                        .long(name)
                        .takes_value(takes)
                        .required(required),
                );
            }
        }
        matches = matches.subcommand(subcmd);
    }

    let matches = matches.get_matches();

    utils::create_r_backups_folder();
    handler::execute(matches).await;
}

use crate::database::client::DBClient;
use colored::*;

pub async fn execute(db: &DBClient) {
    println!("{}", "Counting the databases...".bright_blue());
    let count = db.get_database_count().await;
    println!(
        "{} {}",
        "Number of databases:".bright_blue(),
        count.to_string().green()
    );
}

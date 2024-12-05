use crate::database::client::DBClient;
use colored::*;

pub async fn execute(db: &DBClient) {
    let databases = db.list_databases().await;
    for dtb in databases {
        println!("{}", dtb.green());
    }
}

use std::error::Error;
mod constant;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    constant::constant::check();
    let client = constant::db_connector::get_clients().await?;
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }
    Ok(())
}

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};
use std::env;

use super::constant::get_str;

#[derive(Debug)]
pub enum DBError {
    UserNotfound,
    CollectionNotFound,
    DBNotFound,
    CursorError,
    ClientError,
    ConfigError,
}

pub async fn get_db() -> Result<Database, DBError> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = get_str("MONGGODB_URI");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await;
    let options = match options {
        Ok(v) => v,
        Err(_) => return Err(DBError::ConfigError),
    };

    let client = Client::with_options(options);
    let client = match client {
        Ok(v) => v,
        Err(_) => return Err(DBError::ClientError),
    };
    let db_name =
        get_str("MONGODB_DATABASE");
    let db = client.database(&db_name);
    Ok(db)
}

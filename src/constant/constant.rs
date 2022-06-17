use dotenv;
use std::env;

pub fn check() {
    dotenv::dotenv().expect("Failed to read .env file");
    println!(
        "Mongo DB Url: {}",
        env::var("MONGODB_URI").expect("Mongo DB Url not found")
    );
    println!(
        "Mongo DB Username: {}",
        env::var("MONGODB_USER").expect("Mongo DB Username not found")
    );
    println!(
        "Mongo DB Port: {}",
        env::var("MONGODB_PORT").expect("Mongo DB Portnot found")
    );
}
pub fn get_str(key: &str) -> String {
    dotenv::dotenv().expect("Failed to read .env file");
    env::var(key).expect(&format!("{} not found", key))
}

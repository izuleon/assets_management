mod users;
mod assets;
mod constant;

use std::error::Error;
use rocket::{self};

use users::routes as user;
use assets::routes as asset;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    init().await;
    let _rocket = rocket::build()
        .mount("/users",user::init())
        .mount("/assets",asset::init())
        .launch()
        .await?;
    Ok(())
}
async fn init() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // constant::constant::check();
    // let client = constant::db_connector::get_db().await?;
    Ok(())
}

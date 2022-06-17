use bson::doc;
use mongodb::{Collection, Cursor};
use rocket::futures::{TryFutureExt, TryStreamExt};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::constant::constant::get_str;
use crate::constant::db_connector::{get_db, DBError};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    username: String,
    password: String,
}
impl User {
    async fn get(user_id: &str) -> Result<User, DBError> {
        let db = get_db().await.or_else(|_| return Err(DBError::DBNotFound));
        let coll = db.unwrap().collection::<User>( &get_str("USER_COLLECTION"));
        let user = coll
            .find_one(doc! {"user_id" :user_id}, None)
            .await
            .or_else(|_| return Err(DBError::CursorError));
        user.unwrap().ok_or(DBError::UserNotfound)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            true
        }
        match req.headers().get_one("x-api-key") {
            None => Outcome::Forward(()),
            Some(key) if is_valid(key) => {
                let user = User::get(key).await;
                match user {
                    Ok(val) => Outcome::Success(val),
                    Err(e) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid(e))),
                }
            }
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        }
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid(DBError),
}

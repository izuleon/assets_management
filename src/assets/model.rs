use bson::{doc, Document};
use mongodb::options::FindOptions;
use rocket::futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::constant::constant::get_str;
use crate::constant::db_connector::{get_db, DBError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    asset_id: String,
    name: String,
    description: String,
    attribute: Vec<Document>,
}

impl Asset {
    pub async fn get(asset_id: &str) -> Result<Asset, DBError> {
        let db = get_db().await.or_else(|_| return Err(DBError::DBNotFound));
        let coll = db
            .unwrap()
            .collection::<Asset>(&get_str("ASSET_COLLECTION"));
        let asset = coll
            .find_one(doc! {"asset_id" :asset_id}, None)
            .await
            .or_else(|_| return Err(DBError::CursorError));
        asset.unwrap().ok_or(DBError::UserNotfound)
    }

    pub async fn get_owned_assets(user_id: &str) -> Result<Vec<Asset>, DBError> {
        let db = get_db().await.or_else(|_| return Err(DBError::DBNotFound));
        let coll = db
            .unwrap()
            .collection::<Asset>(&get_str("ASSET_COLLECTION"));
        let cursor = coll
            .find(doc! {"created_by" :user_id}, None)
            .await
            .or_else(|_| return Err(DBError::CursorError));
        cursor
            .unwrap()
            .try_collect()
            .await
            .or_else(|_| return Err(DBError::UserNotfound))
    }

    pub async fn get_by_attr(attribute: Vec<(&str, &str, &str)>) -> Result<Vec<Asset>, DBError> {
        let db = get_db().await.or_else(|_| return Err(DBError::DBNotFound));
        let coll = db
            .unwrap()
            .collection::<Asset>(&get_str("ASSET_COLLECTION"));
        let filter = doc! {
            "$and" :[
                {"asset_id":{"$eq":"asdf"}},
                {"name":{"$eq":"izulasset"}}
            ]
        };
        let filter_option = FindOptions::builder().build();
        let cursor = coll
            .find(filter, filter_option)
            .await
            .or_else(|_| return Err(DBError::CursorError));
        cursor
            .unwrap()
            .try_collect()
            .await
            .or_else(|_| return Err(DBError::UserNotfound))
    }
}

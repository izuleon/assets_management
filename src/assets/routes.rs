use rocket::{delete, form::FromForm, get, post, put, routes, Route};

use crate::users::model::User;

use super::model::Asset;

#[get("/")]
pub async fn asset_list_rt(user: User) -> String {
    format!("{:#?}", Asset::get_owned_assets(&user.user_id).await)
}
#[get("/", rank = 2)]
pub fn asset_list_rt_non_user() -> String {
    "non user assets".to_string()
}

#[post("/")]
pub fn new_asset_rt() -> String {
    "Creation of new asset".to_string()
}

#[derive(Debug, PartialEq, FromForm)]
pub struct Attribute {
    data: String,
    operator: String,
    modifier: String,
}
#[get("/?<attr>&<test>")]
pub async fn info_asset_rt(attr: Option<String>, test: Option<String>) -> String {
    format!(
        "Info for asset {:#?} {:#?}",
        // Asset::get_by_attr(vec![("abc", "abc", "abc")]).await
        attr,
        test
    )
}

#[put("/<id>")]
pub fn update_asset_rt(id: String) -> String {
    format!("Update info for asset {}", id)
}

#[delete("/<id>")]
pub fn delete_asset_rt(id: String) -> String {
    format!("Delete asset {}", id)
}

pub fn init() -> Vec<Route> {
    routes![
        asset_list_rt,
        asset_list_rt_non_user,
        new_asset_rt,
        info_asset_rt,
        update_asset_rt,
        delete_asset_rt
    ]
}

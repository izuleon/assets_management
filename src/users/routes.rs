use rocket::{get, routes, Route, post, put, delete};

#[get("/")]
pub fn user_list_rt() -> String {
    "List of users".to_string()
}

#[post("/")]
pub fn new_user_rt() -> String {
    "Creation of new user".to_string()
}

#[get("/<id>")]
pub fn info_user_rt(id: String) -> String {
    format!("Info for user {}", id)
}

#[put("/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[delete("/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}

pub fn init() -> Vec<Route> {
    routes![
        user_list_rt,
        new_user_rt,
        info_user_rt,
        update_user_rt,
        delete_user_rt
    ]
}

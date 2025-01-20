use actix_web::{put, HttpResponse, Responder};
use crate::model::User;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

#[put("/update")]
pub async fn update_item() -> impl Responder {
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == 1) {
        user.name = "Updated Name".to_string();
        HttpResponse::Ok().body("User updated")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}
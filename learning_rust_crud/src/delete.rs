use actix_web::{delete, HttpResponse, Responder};
use crate::model::User;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

#[delete("/delete")]
pub async fn delete_item() -> impl Responder {
    let mut users = USERS.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == 1) {
        users.remove(pos);
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

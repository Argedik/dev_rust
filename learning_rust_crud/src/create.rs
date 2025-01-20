use actix_web::{post, HttpResponse, Responder};
use crate::model::User;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

#[post("/create")]
pub async fn create_item() -> impl Responder {
    let mut users = USERS.lock().unwrap();
    users.push(User { id: 1, name: "John Doe".to_string() });
    HttpResponse::Ok().body("User created")
}

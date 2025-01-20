use actix_web::{get, HttpResponse, Responder};
use crate::model::User;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

#[get("/read")]
pub async fn get_items() -> impl Responder {
    let users = USERS.lock().unwrap();
    let response = serde_json::to_string(&*users).unwrap();
    HttpResponse::Ok().body(response)
}
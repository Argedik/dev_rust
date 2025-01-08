use axum::{
  extract::{Extension, Path},
  http::StatusCode,
  routing::{get, post, put, delete},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

mod db;
mod models;

use db::Database;
use models::User;

#[derive(Deserialize)]
struct CreateUserRequest {
  name: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
  name: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
  success: bool,
  data: Option<T>,
  message: Option<String>,
}

#[tokio::main]
async fn main() {
  // Veritabanı bağlantısını başlat
  let database_url = "sqlite://my_db.sqlite";
  let db = Database::new(database_url)
      .await
      .expect("Veritabanı bağlantısı kurulamadı");

  // Route’larımızı tanımlayalım
  let app = Router::new()
      // CREATE ve READ
      .route("/users", post(create_user).get(get_users))
      // UPDATE ve DELETE
      .route("/users/:id", put(update_user).delete(delete_user))
      // CORS vb. orta katman ayarları
      .layer(
          CorsLayer::new()
              .allow_origin(Any)
              .allow_methods(Any)
              .allow_headers(Any),
      )
      .layer(Extension(db));

  println!("Sunucu 127.0.0.1:3000 adresinde çalışıyor...");
  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}

// CREATE
async fn create_user(
  Extension(db): Extension<Database>,
  Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<ApiResponse<User>>) {
  match db.create_user(&payload.name).await {
      Ok(user) => (
          StatusCode::CREATED,
          Json(ApiResponse {
              success: true,
              data: Some(user),
              message: None,
          }),
      ),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ApiResponse {
              success: false,
              data: None,
              message: Some(e.to_string()),
          }),
      ),
  }
}

// READ
async fn get_users(
  Extension(db): Extension<Database>,
) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
  match db.get_users().await {
      Ok(users) => (
          StatusCode::OK,
          Json(ApiResponse {
              success: true,
              data: Some(users),
              message: None,
          }),
      ),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ApiResponse {
              success: false,
              data: None,
              message: Some(e.to_string()),
          }),
      ),
  }
}

// UPDATE
async fn update_user(
  Extension(db): Extension<Database>,
  Path(id): Path<i64>,
  Json(payload): Json<UpdateUserRequest>,
) -> (StatusCode, Json<ApiResponse<User>>) {
  match db.update_user(id, &payload.name).await {
      Ok(updated_user) => (
          StatusCode::OK,
          Json(ApiResponse {
              success: true,
              data: Some(updated_user),
              message: None,
          }),
      ),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ApiResponse {
              success: false,
              data: None,
              message: Some(e.to_string()),
          }),
      ),
  }
}

// DELETE
async fn delete_user(
  Extension(db): Extension<Database>,
  Path(id): Path<i64>,
) -> (StatusCode, Json<ApiResponse<()>>) {
  match db.delete_user(id).await {
      Ok(_) => (
          StatusCode::OK,
          Json(ApiResponse {
              success: true,
              data: Some(()),
              message: Some("Kullanıcı silindi".to_string()),
          }),
      ),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ApiResponse {
              success: false,
              data: None,
              message: Some(e.to_string()),
          }),
      ),
  }
}

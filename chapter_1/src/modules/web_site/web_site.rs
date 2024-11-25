use actix_web::{web, App, HttpServer, Responder};

#[allow(dead_code)]
pub async fn index() -> impl Responder {
      let html = r#"
      <!DOCTYPE html>
      <html lang="tr">
      <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>Merhaba</title>
          <style>
              body {
                  display: flex;
                  justify-content: center;
                  align-items: center;
                  height: 100vh;
                  margin: 0;
                  font-family: Arial, sans-serif;
                  background-color: #f0f0f0;
              }
              h1 {
                  color: #333;
              }
          </style>
      </head>
      <body>
          <h1>Merhaba</h1>
      </body>
      </html>
      "#;

      actix_web::HttpResponse::Ok()
          .content_type("text/html; charset=utf-8")
          .body(html)
  }

  #[actix_web::main]
  async fn main() -> std::io::Result<()> {
      HttpServer::new(|| {
          App::new()
              .route("/", web::get().to(index))
      })
      .bind("127.0.0.1:8080")?
      .run()
      .await
    }
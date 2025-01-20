use actix_web::{web, App, HttpServer};
mod create;
mod read;
mod update;
mod delete;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create::create_item)
            .service(read::get_items)
            .service(update::update_item)
            .service(delete::delete_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
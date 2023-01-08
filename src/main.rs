#[macro_use]
extern crate diesel;
mod controllers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(controllers::general_controllers::index))
            .route(
                "/users",
                web::get().to(controllers::user_controllers::list_users),
            )
            .route(
                "/users/{id}",
                web::get().to(controllers::user_controllers::get_user),
            )
            .route(
                "/users/{id}",
                web::patch().to(controllers::user_controllers::update_user),
            )
            .route(
                "/users/{id}",
                web::delete().to(controllers::user_controllers::delete_user),
            )
            .route(
                "/users",
                web::post().to(controllers::user_controllers::create_user),
            )
    })
    .bind("localhost:8080")?
    .run()
    .await
}

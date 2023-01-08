#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod controllers;
mod db;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    db::init();

    HttpServer::new(move || {
        App::new()
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

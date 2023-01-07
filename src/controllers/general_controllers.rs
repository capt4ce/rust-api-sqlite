use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse};
use anyhow::Result;

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world Rust!"))
}

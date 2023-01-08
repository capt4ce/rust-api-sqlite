use crate::models::user_models::{User, Users};

use actix_web::{web, Error, HttpResponse};
use anyhow::Result;

pub async fn list_users() -> Result<HttpResponse, Error> {
    let result = Users::list().unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn create_user(data: web::Json<User>) -> Result<HttpResponse, Error> {
    let result = Users::create(data.into_inner()).unwrap();
    Ok(HttpResponse::Created().json(result))
}

pub async fn get_user(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result = Users::get(id.into_inner()).unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_user(id: web::Path<i32>, data: web::Json<User>) -> Result<HttpResponse, Error> {
    let result = Users::update(id.into_inner(), data.into_inner()).unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Users::delete(id.into_inner()).unwrap();
    Ok(HttpResponse::Ok().finish())
}

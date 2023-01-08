use crate::models::user_models::{User, UserDataJson, UserNew};
use crate::Pool;

use crate::schema::users;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub async fn list_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let users_result = users.load::<User>(&db_connection);
    Ok(users_result
        .map(|users_result| HttpResponse::Ok().json(users_result))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create_user(
    pool: web::Data<Pool>,
    data: web::Json<UserDataJson>,
) -> Result<HttpResponse, Error> {
    let db_connection = pool.get().unwrap();
    let user_name = match &data.name {
        Some(name_data) => name_data,
        None => "",
    };
    let user_address = match &data.address {
        Some(address_data) => address_data,
        None => "",
    };
    match users::table
        .filter(users::name.eq(&user_name))
        .first::<User>(&db_connection)
    {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(_) => {
            let new_user = UserNew {
                name: &user_name,
                address: &user_address,
                created_at: &format!("{}", chrono::Local::now().naive_local()),
            };

            diesel::insert_into(users::table)
                .values(new_user)
                .execute(&db_connection)
                .expect("User insert fails");
            let result: User = users::table
                .order(users::id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(HttpResponse::Created().json(result))
        }
    }
}

pub async fn get_user(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db_connection = pool.get().unwrap();
    let result = users::table
        .filter(users::id.eq(id.into_inner()))
        .first::<User>(&db_connection)
        .unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_user(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    data: web::Json<User>,
) -> Result<HttpResponse, Error> {
    let db_connection = pool.get().unwrap();
    let user_id = id.into_inner();
    diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(data.into_inner())
        .execute(&db_connection)
        .unwrap();
    let result = users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&db_connection)
        .unwrap();

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_user(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let db_connection = pool.get().unwrap();
    diesel::delete(users::table)
        .filter(users::id.eq(id.into_inner()))
        .execute(&db_connection);

    Ok(HttpResponse::Ok().finish())
}

use crate::models::{User, UserJson, UserNew};
use crate::Pool;

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world Rust!"))
}

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
    data: web::Json<UserJson>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    match users
        .filter(name.eq(&data.name))
        .first::<User>(&db_connection)
    {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(_) => {
            let new_user = UserNew {
                name: &data.name,
                address: &data.address,
                created_at: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(users)
                .values(new_user)
                .execute(&db_connection)
                .expect("User insert fails");
            let result: User = users.order(id.desc()).first(&db_connection).unwrap();
            Ok(HttpResponse::Created().json(result))
        }
    }
}

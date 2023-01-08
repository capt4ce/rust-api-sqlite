use crate::db;
use crate::schema::users;
use anyhow::Result;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub created_at: &'a str,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub created_at: String,
}

impl Users {
    pub fn get(id: i32) -> Result<Users> {
        let conn = db::connection()?;
        let result = users::table
            .filter(users::id.eq(id))
            .first::<Users>(&conn)?;
        Ok(result)
    }

    pub fn list() -> Result<Vec<Users>> {
        let conn = db::connection()?;
        let result = users::table.load::<Users>(&conn)?;
        Ok(result)
    }

    pub fn create(data: User) -> Result<Users> {
        let conn = db::connection()?;
        let new_user = UserNew {
            name: &data.name,
            address: &data.address,
            created_at: &format!("{}", chrono::Local::now().naive_local()),
        };

        match users::table
            .filter(users::name.eq(&new_user.name))
            .first::<Users>(&conn)
        {
            Ok(result) => Ok(result),
            Err(_) => {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .execute(&conn)?;
                let result = users::table.order(users::id.desc()).first::<Users>(&conn)?;
                Ok(result)
            }
        }
    }

    pub fn update(id: i32, data: User) -> Result<Users> {
        let conn = db::connection()?;
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(data)
            .execute(&conn)?;
        let result = users::table
            .filter(users::id.eq(id))
            .first::<Users>(&conn)?;
        Ok(result)
    }

    pub fn delete(id: i32) -> Result<()> {
        let conn = db::connection()?;
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(&conn)?;
        Ok(())
    }
}

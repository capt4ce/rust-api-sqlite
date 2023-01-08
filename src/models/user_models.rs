use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub created_at: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataJson {
    pub name: Option<String>,
    pub address: Option<String>,
}

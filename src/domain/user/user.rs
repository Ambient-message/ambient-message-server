use diesel::{Queryable, QueryableByName, Selectable, Insertable};
use uuid::Uuid;
use crate::adapters::spi::db::schema::*;

#[derive(Debug, Clone)]
#[derive(Queryable, Selectable)]
#[derive(Insertable)]
#[diesel(table_name = crate::adapters::spi::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(id : Uuid, username : &str, password : &str) -> Self{
        Self{
            id : id,
            username : username.to_string(),
            password : password.to_string()
        }
    }
}

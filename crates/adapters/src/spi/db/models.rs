use super::schema::users;
use uuid::Uuid;
use diesel::{prelude::*};

#[derive(Queryable, Selectable)]
#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}
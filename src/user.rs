use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser{
    pub username: String,
    pub password: String,
}
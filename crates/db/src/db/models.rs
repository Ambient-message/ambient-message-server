use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}
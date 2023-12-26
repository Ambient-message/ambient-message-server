use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Insertable, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

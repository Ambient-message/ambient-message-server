use diesel::{QueryableByName, Queryable, Insertable, Selectable};
use uuid::Uuid;

use crate::spi::schema::*;

#[derive(Insertable, Selectable, Queryable, QueryableByName)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

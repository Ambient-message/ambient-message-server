use crate::domain::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Request  {
    GetUser(i32),
    AddUser(User),
}
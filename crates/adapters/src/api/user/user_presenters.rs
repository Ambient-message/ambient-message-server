use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub id: Uuid,
    pub username: String,
}

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct ChatEntity {
    pub id: Uuid,
    pub created: DateTime<Utc>,
}

impl ChatEntity {
    pub fn new() -> Self
    {
        Self { id: Uuid::new_v4(), created: chrono::offset::Utc::now() }
    }
}
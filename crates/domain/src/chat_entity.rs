use uuid::Uuid;

#[derive(Debug)]
pub struct ChatEntity{
    pub id: Uuid
}

impl ChatEntity {
    pub fn new() -> Self
    {
        Self { id: Uuid::new_v4(), }
    }
}
#[derive(Debug)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new<N>(id: i32, name: N) -> Self
    where N: Into<String>
    {
        Self {
            id,
            name: name.into(),
        }
    }
}

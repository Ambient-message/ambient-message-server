use domain::entities::user::User;

pub struct Record {
    pub user: User,
}

pub trait UserRepository {
    fn save(&self, record: Record) -> Result<User, ()>;
}
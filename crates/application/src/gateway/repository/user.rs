use domain::entity::user::User;

pub struct Record {
    pub user: User,
}

pub trait Repo {
    fn save(&self, record: Record) -> Result<User, ()>;
}

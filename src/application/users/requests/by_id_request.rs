use crate::domain::user::User;
use crate::domain::user::user_repository_abstract::UserRepositoryAbstract;

pub struct ByIdRequest {
    user_repository: Box<dyn UserRepositoryAbstract>,
}

impl ByIdRequest {
    pub fn new(user_repository: Box<dyn UserRepositoryAbstract>) -> Self {
        Self {
            user_repository,
        }
    }

    pub fn handler(&self, id: i32) -> Result<User, String> {
        self.user_repository.by_id(id)
    }
}
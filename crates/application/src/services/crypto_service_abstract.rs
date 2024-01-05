use jsonwebtoken::TokenData;
use uuid::Uuid;

use domain::api_error::ApiError;
use domain::claims::Claims;

pub trait CryptoServiceAbstract {
    async fn hash_password(&self, password: String) -> Result<String, ApiError>;
    async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, ApiError>;
    async fn generate_jwt(&self, user_id: Uuid) -> Result<String, ApiError>;
    async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>, ApiError>;
}

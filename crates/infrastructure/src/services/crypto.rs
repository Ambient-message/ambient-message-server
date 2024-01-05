use actix_web::web::block;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;

use application::services::crypto_service_abstract::CryptoServiceAbstract;
use domain::api_error::ApiError;
use domain::claims::Claims;

pub struct CryptoService {
    pub jwt_secret: String,
}


impl CryptoServiceAbstract for CryptoService {
    //TODO make async
    async fn hash_password(&self, password: String) -> Result<String, ApiError> {
        hash(password, bcrypt::DEFAULT_COST).map_err(|err| ApiError {
            code: 400,
            message: String::from("Can't hash the password"),
            error: Box::new(err),
        })
    }

    //TODO make async
    async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, ApiError> {
        verify(password, password_hash).map_err(|err| ApiError {
            code: 400,
            message: String::from("Verifying error").into(),
            error: Box::new(err),
        })
    }

    //TODO make async like this
    async fn generate_jwt(&self, user_id: Uuid) -> Result<String, ApiError> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let now = Utc::now() + Duration::days(1); // Expires in 1 day
            let claims = Claims {
                sub: user_id.into(),
                exp: now.timestamp(),
            };
            encode(&headers, &claims, &encoding_key).unwrap()
        })
            .await
            .map_err(|err| ApiError {
                code: 400,
                message: String::from("Can't creating jwt token").into(),
                error: Box::new(err),
            })
    }

    async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>, ApiError> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation).map_err(|err| ApiError {
                code: 400,
                message: String::from("Can't verifying jwt token").into(),
                error: Box::new(err),
            })
        })
            .await
            .map_err(|err| ApiError {
                code: 400,
                message: String::from("Can't verifying jwt token").into(),
                error: Box::new(err),
            })
            .and_then(|result| result)
    }
}
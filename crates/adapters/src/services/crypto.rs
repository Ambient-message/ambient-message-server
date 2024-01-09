use actix_web::http::StatusCode;
use actix_web::web::block;
use bcrypt::hash;
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
        hash(password, bcrypt::DEFAULT_COST).map_err(|err| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't hasp password",
                err,
            )
        })
    }

    //TODO make async
    async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, ApiError> {
        Ok(password == password_hash)
        // verify(password, password_hash).map_err(|err| ApiError {
        //     code: 400,
        //     message: String::from("Verifying error"),
        //     error: Box::new(err),
        // })
    }

    //TODO make async like this
    async fn generate_jwt(&self, user_id: Uuid) -> Result<String, ApiError> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let now = Utc::now() + Duration::days(1); // Expires in 1 day
            let claims = Claims {
                sub: user_id,
                exp: now.timestamp(),
            };
            encode(&headers, &claims, &encoding_key).map_err(|err| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Can't encode the token",
                    err,
                )
            })
        })
            .await
            .map_err(|err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Blocking error", err))?
    }

    async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>, ApiError> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation).map_err(|err| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Can't decode the token",
                    err,
                )
            })
        })
            .await
            .map_err(|err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Blocking error", err))?
    }
}

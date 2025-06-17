use crate::database::models::user::User;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

pub fn hash_bytes(bytes: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2.hash_password(bytes, &salt)?;
    Ok(hash.to_string())
}

pub fn verify_bytes(bytes: &[u8], hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(bytes, &parsed_hash)
        .is_ok())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl JWTClaims {
    pub fn get_uuid(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
}

pub fn generate_jwt(
    user: &User,
    secret: &str,
    validity_duration: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = now + validity_duration;

    let claims = JWTClaims {
        sub: user.id.to_string(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };

    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claims, &key)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode(token, &key, &validation)?;
    Ok(token_data.claims)
}

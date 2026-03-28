// src/auth.rs

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set — using insecure fallback. Set it in production!");
        "changeme_in_production".to_string()
    })
}

const TOKEN_EXPIRY_SECS: u64 = 8 * 60 * 60; // 8 hours

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // username
    pub exp: u64,    // expiry  (unix timestamp)
    pub iat: u64,    // issued at (unix timestamp)
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

pub fn create_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: username.to_string(),
        exp: now + TOKEN_EXPIRY_SECS,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.algorithms = vec![jsonwebtoken::Algorithm::HS256];
    validation.validate_exp = true;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        __parts__: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let token = __parts__
                .headers
                .get("Token")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
                .or_else(|| {
                    __parts__
                        .headers
                        .get("cookie")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|cookies| {
                            cookies
                                .split(';')
                                .map(|c| c.trim())
                                .find(|c| c.starts_with("token="))
                                .map(|c| c["token=".len()..].to_string())
                        })
                })
                .ok_or((StatusCode::UNAUTHORIZED, "Missing Token header or cookie"))?;

            decode_token(&token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))
        }
    }
}

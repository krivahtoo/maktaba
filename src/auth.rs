use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::Cookies;

use crate::{
    error::{Error, Result},
    extractors::json::Json,
    model::user::UserRole,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub user_id: i64,
    pub role: UserRole,
    pub exp: usize,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> core::result::Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let token = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(v))) => v.token().to_owned(),
            _ => {
                let cookies = parts
                    .extract::<Cookies>()
                    .await
                    .map_err(|_| AuthError::MissingCredentials)?;
                cookies
                    .get("token")
                    .map(|c| c.value().to_owned())
                    .ok_or(AuthError::MissingCredentials)?
            }
        };
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
        // Decode the user data
        use jsonwebtoken::errors::ErrorKind as E;
        let claims = decode_jwt(&token, &jwt_secret).map_err(|e| match e.kind() {
            E::ExpiredSignature => AuthError::ExpiredToken,
            _ => AuthError::InvalidToken,
        })?;

        Ok(claims)
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidToken,
    ExpiredToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingCredentials => (StatusCode::FORBIDDEN, "Missing credentials"),
            AuthError::InvalidToken => (StatusCode::FORBIDDEN, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::FORBIDDEN, "Expired token. Please login again"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn generate_jwt(claims: &Claims, jwt_secret: &str) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_jwt(
    token: &str,
    jwt_secret: &str,
) -> core::result::Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let hashed = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    Ok(hashed.to_string())
}

pub fn verify(password_hash: &str, password: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| Error::Argon2(e.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| Error::Argon2(e.to_string()))?;

    Ok(())
}

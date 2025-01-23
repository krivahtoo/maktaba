use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::{Error, Result}, model::user::UserRole};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub user_id: i64,
    pub role: UserRole,
    pub exp: usize,
}

pub fn generate_jwt(claims: &Claims, jwt_secret: &str) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_jwt(token: &str, jwt_secret: &str) -> Result<Claims> {
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

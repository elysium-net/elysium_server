use crate::api::user;
use crate::config;
use crate::database::Database;
use crate::error::ElyError;
use elysium_api::user::User;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tonic::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Claim {
    sub: String,
    exp: u64,
}

pub async fn create_token(
    database: Arc<Database>,
    user: String,
    password: String,
) -> Result<String, ElyError> {
    tracing::trace!(
        "create_token with user: {} and password: {}",
        user,
        password
    );

    user::verify(database, user.clone(), password).await?;

    let exp = jsonwebtoken::get_current_timestamp() + *config::app::JWT_EXPIRY_SECS;

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::from_str(config::app::JWT_ALGO.as_str())?),
        &Claim { sub: user, exp },
        &EncodingKey::from_secret(config::app::JWT_KEY.as_bytes()),
    )?;

    Ok(token)
}

pub async fn verify_token(database: Arc<Database>, token: &str) -> Result<User, ElyError> {
    tracing::trace!("verify_token with token: {}", token);

    let token = jsonwebtoken::decode::<Claim>(
        token,
        &DecodingKey::from_secret(config::app::JWT_KEY.as_bytes()),
        &Default::default(),
    )?;

    user::get(database, &token.claims.sub).await
}

pub async fn auth<T>(database: Arc<Database>, req: &Request<T>) -> Result<User, ElyError> {
    let token = req
        .metadata()
        .get("token")
        .ok_or(ElyError::Unauthorized)?
        .to_str()
        .map_err(|_| ElyError::Unauthorized)?;

    verify_token(database, token).await
}

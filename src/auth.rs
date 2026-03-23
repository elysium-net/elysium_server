use crate::database::Database;
use crate::error::Error;
use crate::{cfg, user};
use argon2::password_hash::phc::Salt;
use argon2::{Argon2, Params, PasswordVerifier, Version};
use elysium_rust::common::v1::{Auth, ErrorCode};
use elysium_rust::user::v1::{User, UserRole};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::sync::OnceLock;
use std::time::SystemTime;
use tonic::Request;

const ARGON2_HASH_LEN: usize = 32;

static ARGON2: OnceLock<Argon2> = OnceLock::new();
static KEYS: OnceLock<(EncodingKey, DecodingKey)> = OnceLock::new();

pub fn init() {
    ARGON2
        .set(Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Params::new(19 * 1024, 2, 1, Some(ARGON2_HASH_LEN))
                .expect("Failed to create Argon2 params"),
        ))
        .expect("Failed to initialize Argon2");

    KEYS.set((
        EncodingKey::from_ed_pem(cfg::PRIVATE_AUTH_KEY.as_bytes())
            .expect("Failed to create argon2 encoding key"),
        DecodingKey::from_ed_pem(cfg::PUBLIC_AUTH_KEY.as_bytes())
            .expect("Failed to create argon2 decoding key"),
    ))
    .expect("Failed to initialize argon2 keys");
}

pub async fn verify_role<T>(
    database: &Database,
    req: &Request<T>,
    target: UserRole,
) -> Result<User, Error> {
    let user = verify(database, req).await?;
    let target: i32 = target.into();

    if user.role < target {
        Err(Error::new(
            ErrorCode::Unauthorized,
            "Insufficient permissions",
        ))
    } else {
        Ok(user)
    }
}

pub async fn verify<T>(database: &Database, req: &Request<T>) -> Result<User, Error> {
    let meta = req.metadata();

    let (_, key) = keys();

    if let Some(token) = meta.get("Authorization") {
        let token = String::from_utf8_lossy(token.as_bytes());
        let claim =
            jsonwebtoken::decode::<Auth>(token.as_bytes(), key, &Validation::new(Algorithm::EdDSA))
                .map_err(|_| Error::new(ErrorCode::Unauthorized, "Invalid token"))?;

        let now = SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Failed to get current time")
            .as_secs();

        if claim.claims.exp > now as i64 {
            Err(Error::new(ErrorCode::Unauthorized, "Token expired"))
        } else {
            user::get(database, &claim.claims.userid)
                .await?
                .ok_or(Error::new(ErrorCode::NotFound, "User not found"))
        }
    } else {
        Err(Error::new(ErrorCode::Unauthorized, "Missing token"))
    }
}

pub async fn auth(database: &Database, userid: String, password: String) -> Result<String, Error> {
    let user = user::get(database, &userid).await?;
    let auth = Auth {
        userid,
        exp: SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Failed to get current time")
            .as_secs() as i64,
    };

    let (key, _) = keys();

    if let Some(user) = user {
        if verify_hash(password, user.password) {
            jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), &auth, key)
                .map_err(|_| Error::new(ErrorCode::Internal, "Failed to encode token"))
        } else {
            Err(Error::new(ErrorCode::Unauthorized, "Invalid credentials"))
        }
    } else {
        Err(Error::new(ErrorCode::Unauthorized, "Invalid credentials"))
    }
}

fn hash(pass: String) -> Result<String, argon2::Error> {
    let argon2 = argon2();
    let mut hash = [0; ARGON2_HASH_LEN];

    let salt = Salt::generate();

    argon2.hash_password_into(pass.as_bytes(), &salt, &mut hash)?;

    Ok(String::from_utf8_lossy(pass.as_bytes()).to_string())
}

fn verify_hash(pass: String, hash: String) -> bool {
    argon2()
        .verify_password(pass.as_bytes(), hash.as_str())
        .is_ok()
}

fn argon2<'a>() -> &'a Argon2<'a> {
    ARGON2.get().expect("Argon2 not initialized yet")
}

fn keys<'a>() -> &'a (EncodingKey, DecodingKey) {
    KEYS.get().expect("Keys not initialized yet")
}

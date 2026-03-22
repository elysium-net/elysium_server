use crate::database::Database;
use crate::error::Error;
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::user::v1::{User, UserProfile};

pub const TABLE: &str = "user";

pub async fn create(database: &Database, user: User) -> Result<(), Error> {
    if exists(database, user.id.as_str()).await? {
        return Err(Error::new(ErrorCode::AlreadyExists, "User already exists"));
    }

    let _: Option<User> = database
        .create((TABLE, user.id.as_str()))
        .content(user)
        .await?;

    Ok(())
}

pub async fn delete(database: &Database, userid: &str) -> Result<Option<()>, Error> {
    if exists(database, userid).await? {
        let _: Option<User> = database.delete((TABLE, userid)).await?;

        Ok(Some(()))
    } else {
        Ok(None)
    }
}

pub async fn get(database: &Database, userid: &str) -> Result<Option<User>, Error> {
    let result: Option<User> = database.select((TABLE, userid)).await?;

    Ok(result)
}

pub async fn exists(database: &Database, userid: &str) -> Result<bool, Error> {
    Ok(get(database, userid).await?.is_some())
}

pub fn to_profile(user: User) -> UserProfile {
    UserProfile {
        id: user.id,
        username: user.username,
        role: user.role,
        icon: user.icon,
    }
}

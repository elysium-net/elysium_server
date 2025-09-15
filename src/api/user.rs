use crate::database::Database;
use crate::error::ElyError;
use elysium_api::user::User;
use std::sync::Arc;
use surrealdb::RecordId;

pub async fn create(database: Arc<Database>, user: User) -> Result<User, ElyError> {
    if exists(database.clone(), &user.name).await? {
        return Err(ElyError::UserAlreadyExists);
    }

    let user: Option<User> = database
        .create(("user", user.name.as_str()))
        .content(user)
        .await?;

    Ok(user.unwrap())
}

pub async fn get(database: Arc<Database>, id: &String) -> Result<User, ElyError> {
    let user = database.select(("user", id.as_str())).await?;

    user.ok_or(ElyError::UserNotFound)
}

pub async fn exists(database: Arc<Database>, id: &String) -> Result<bool, ElyError> {
    let user = get(database, id).await;

    match user {
        Ok(_) => Ok(true),
        Err(ElyError::UserNotFound) => Ok(false),
        Err(e) => Err(e),
    }
}

pub async fn delete(database: Arc<Database>, id: &String) -> Result<User, ElyError> {
    let user: Option<User> = database.delete(("user", id.as_str())).await?;

    user.ok_or(ElyError::UserNotFound)
}

pub async fn verify(
    database: Arc<Database>,
    user: String,
    password: String,
) -> Result<(), ElyError> {
    let user = get(database, &user).await?;

    if user.password.verify(password) {
        Ok(())
    } else {
        Err(ElyError::UserNotFound)
    }
}

pub async fn update(database: Arc<Database>, user: User) -> Result<User, ElyError> {
    let user: Option<User> = database
        .update(("user", user.name.as_str()))
        .content(user)
        .await?;

    user.ok_or(ElyError::UserNotFound)
}

pub async fn follow(
    database: Arc<Database>,
    user: String,
    follows: String,
) -> Result<(), ElyError> {
    if !exists(database.clone(), &user).await? {
        return Err(ElyError::UserNotFound);
    }

    database
        .query("RELATE $user->follows->$follows")
        .bind(("user", RecordId::from_table_key("user", user)))
        .bind(("follows", RecordId::from_table_key("user", follows)))
        .await?
        .check()?;

    Ok(())
}

pub async fn unfollow(
    database: Arc<Database>,
    user: String,
    follows: String,
) -> Result<(), ElyError> {
    if !exists(database.clone(), &user).await? {
        return Err(ElyError::UserNotFound);
    }

    database
        .query("DELETE $user->follows WHERE out=$follows")
        .bind(("user", RecordId::from_table_key("user", user)))
        .bind(("follows", RecordId::from_table_key("user", follows)))
        .await?
        .check()?;

    Ok(())
}

pub async fn get_follows(database: Arc<Database>, user: String) -> Result<Vec<User>, ElyError> {
    let follows: Vec<User> = database
        .query("SELECT * FROM(SELECT ->follows->user AS user FROM $user)[0].user")
        .bind(("user", RecordId::from_table_key("user", user)))
        .await?
        .take(0)?;

    Ok(follows)
}

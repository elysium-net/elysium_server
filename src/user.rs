use crate::database::Database;
use crate::error::Error;
use crate::{auth, cfg};
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::user::v1::{User, UserProfile, UserRole};
use std::collections::HashMap;
use surrealdb::Notification;
use surrealdb::method::QueryStream;

pub async fn create(database: &Database, user: User) -> Result<(), Error> {
    if exists(database, user.user_id.as_str()).await? {
        return Err(Error::new(ErrorCode::AlreadyExists, "User already exists"));
    }

    let _: Option<User> = database
        .create(("user", user.user_id.as_str()))
        .content(user)
        .await?;

    Ok(())
}

pub async fn delete(database: &Database, userid: &str) -> Result<Option<()>, Error> {
    if exists(database, userid).await? {
        let _: Option<User> = database.delete(("user", userid)).await?;

        Ok(Some(()))
    } else {
        Ok(None)
    }
}

pub async fn update(database: &Database, user: User) -> Result<(), Error> {
    if exists(database, &user.user_id).await? {
        let _: Option<User> = database
            .update(("user", user.user_id.as_str()))
            .content(user)
            .await?;

        Ok(())
    } else {
        Err(Error::new(ErrorCode::NotFound, "User not found"))
    }
}

pub async fn get(database: &Database, userid: &str) -> Result<Option<User>, Error> {
    let result: Option<User> = database.select(("user", userid)).await?;

    Ok(result)
}

pub async fn search(database: &Database, query: String) -> Result<Vec<UserProfile>, Error> {
    let results = database
        .query(
            r#"
        SELECT * FROM user
        WHERE user_id CONTAINS $query
           OR username CONTAINS $query
        LIMIT $limit
    "#,
        )
        .bind(("query", query))
        .bind(("limit", *cfg::MAX_SEARCH_RESULTS))
        .await?
        .take::<Vec<User>>(0)?;

    Ok(results.into_iter().map(to_profile).collect())
}

pub async fn exists(database: &Database, userid: &str) -> Result<bool, Error> {
    Ok(get(database, userid).await?.is_some())
}

pub fn to_profile(user: User) -> UserProfile {
    UserProfile {
        user_id: user.user_id,
        username: user.username,
        role: user.role,
        icon: user.icon,
    }
}

pub async fn create_admin(database: &Database) -> Result<(), Error> {
    if exists(database, "admin").await? {
        match auth::auth(database, "admin".to_string(), "admin".to_string()).await {
            Ok(_) => tracing::warn!(
                "The initial 'admin' user has an unsecure password. Please change this immediately!"
            ),

            Err(err) => {
                if err.code() != ErrorCode::Unauthorized {
                    return Err(err);
                }
            }
        }
    } else {
        create(
            database,
            User {
                user_id: "admin".to_string(),
                username: "admin".to_string(),
                email: "".to_string(),
                password: auth::hash("admin".to_string()).expect("Failed to hash password"),
                role: UserRole::Admin as i32,
                icon: None,
            },
        )
        .await?;

        tracing::info!(
            "Created setup administrator 'admin' with password 'admin'. Please change this immediately!"
        );
    }

    Ok(())
}

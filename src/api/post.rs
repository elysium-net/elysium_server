use crate::api::user;
use crate::database::Database;
use crate::error::ElyError;
use elysium_api::post::{Post, PostId};
use elysium_api::user::User;
use elysium_api::{algo, post};
use std::sync::Arc;
use surrealdb::RecordId;

pub async fn create(database: Arc<Database>, author: User, post: Post) -> Result<PostId, ElyError> {
    tracing::trace!(
        "create post by user {} with content {:?}",
        author.name,
        post.inner.content
    );

    let id = post::generate_id();

    let post: Option<Post> = database.create(("post", id.as_str())).content(post).await?;
    post.unwrap();

    database
        .query("RELATE $user->posted->$post")
        .bind(("user", RecordId::from_table_key("user", author.name)))
        .bind(("post", RecordId::from_table_key("post", &id)))
        .await?
        .check()?;

    Ok(id)
}

pub async fn delete(database: Arc<Database>, id: &PostId) -> Result<Post, ElyError> {
    tracing::trace!("delete post: {}", id);

    let post: Option<Post> = database
        .delete(RecordId::from_table_key("post", id))
        .await?;

    post.ok_or(ElyError::PostNotFound)
}

pub async fn get(database: Arc<Database>, id: &PostId) -> Result<Post, ElyError> {
    tracing::trace!("get post: {}", id);

    let post: Option<Post> = database.select(("post", id)).await?;

    post.ok_or(ElyError::PostNotFound)
}

pub async fn get_author(database: Arc<Database>, post: PostId) -> Result<User, ElyError> {
    tracing::trace!("get author of post: {}", post);

    let user: Option<User> = database
        .query("SELECT * FROM(SELECT <-posted<-user AS user FROM $post)[0].user")
        .bind(("post", RecordId::from_table_key("post", post)))
        .await?
        .take(0)?;

    Ok(user.unwrap())
}

pub async fn get_recommendations(
    database: Arc<Database>,
    user: User,
) -> Result<Vec<PostId>, ElyError> {
    tracing::trace!("get recommendations for user: {}", user.name);

    let follows = user::get_follows(database.clone(), user.name.clone()).await?;

    let user_vec =
        algo::post::build_user_vector(&user, follows.into_iter().map(|u| u.name).collect());

    database
        .query(
            "DEFINE INDEX IF NOT EXISTS mt_post ON post FIELDS vector MTREE DIMENSION 4 DIST COSINE TYPE F64"
        )
        .await?
        .check()?;

    let results: Vec<RecordId> = database.query(
        "SELECT id, vector::similarity::cosine(vector, $user_vec) AS dist FROM post WHERE vector <|4|> $user_vec ORDER BY dist"
    )
        .bind(("user_vec", user_vec))
        .await?
        .take((0, "id"))?;

    Ok(results.into_iter().map(|id| id.key().to_string()).collect())
}

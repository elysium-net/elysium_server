use crate::database::Database;
use crate::error::ElyError;
use elysium_api::post::PostId;
use std::sync::Arc;
use surrealdb::RecordId;

pub async fn on(database: Arc<Database>, comment: PostId, on_post: PostId) -> Result<(), ElyError> {
    tracing::trace!("comment on post: {} with comment: {}", on_post, comment);

    database
        .query("RELATE $comment->commented->$post")
        .bind(("comment", RecordId::from_table_key("post", comment)))
        .bind(("post", RecordId::from_table_key("post", on_post)))
        .await?
        .check()?;

    Ok(())
}

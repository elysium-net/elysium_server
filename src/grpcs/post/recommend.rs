use crate::api::{auth, post};
use crate::state::ServerState;
use elysium_api::proto::{RecommendPostRequest, RecommendPostResponse};
use tonic::{Request, Response, Status};

pub async fn recommend_post(
    state: ServerState,
    req: Request<RecommendPostRequest>,
) -> Result<Response<RecommendPostResponse>, Status> {
    let user = auth::auth(state.database(), &req).await?;

    let post_ids = post::get_recommendations(state.database(), user)
        .await
        .expect("failed to get recommendation");

    let mut posts = Vec::with_capacity(post_ids.capacity());

    for id in post_ids {
        let post = post::get(state.database(), &id).await?;

        // TODO: also push post author for less requests
        posts.push(post.inner);
    }

    Ok(Response::new(RecommendPostResponse { posts }))
}

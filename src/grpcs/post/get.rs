use crate::api::post;
use crate::state::ServerState;
use elysium_api::proto::{GetPostRequest, GetPostResponse};
use tonic::{Request, Response, Status};

pub async fn get_post(
    state: ServerState,
    req: Request<GetPostRequest>,
) -> Result<Response<GetPostResponse>, Status> {
    let req = req.into_inner();
    let post = post::get(state.database(), &req.id).await?;

    Ok(Response::new(GetPostResponse {
        post: Some(post.inner),
    }))
}

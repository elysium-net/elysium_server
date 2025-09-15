use crate::api::post;
use crate::state::ServerState;
use elysium_api::proto::{GetPostAuthorRequest, GetPostAuthorResponse};
use tonic::{Request, Response, Status};

pub async fn get_post_author(
    state: ServerState,
    req: Request<GetPostAuthorRequest>,
) -> Result<Response<GetPostAuthorResponse>, Status> {
    let user = post::get_author(state.database(), req.into_inner().id).await?;

    Ok(Response::new(GetPostAuthorResponse {
        user: Some(user.profile()),
    }))
}

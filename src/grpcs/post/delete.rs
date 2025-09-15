use crate::api::{auth, post};
use crate::state::ServerState;
use elysium_api::proto::{DeletePostRequest, DeletePostResponse};
use tonic::{Request, Response, Status};

pub async fn delete_post(
    state: ServerState,
    req: Request<DeletePostRequest>,
) -> Result<Response<DeletePostResponse>, Status> {
    auth::auth(state.database(), &req).await?;

    post::delete(state.database(), &req.into_inner().id).await?;

    Ok(Response::new(DeletePostResponse {}))
}

use crate::api::user;
use crate::state::ServerState;
use elysium_api::proto::{DeleteUserRequest, DeleteUserResponse};
use tonic::{Request, Response, Status};

pub async fn delete_user(
    state: ServerState,
    req: Request<DeleteUserRequest>,
) -> Result<Response<DeleteUserResponse>, Status> {
    let req = req.into_inner();

    user::verify(state.database(), req.name.clone(), req.password).await?;

    user::delete(state.database(), &req.name).await?;

    Ok(Response::new(DeleteUserResponse {}))
}

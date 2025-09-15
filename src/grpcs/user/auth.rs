use crate::api::auth;
use crate::state::ServerState;
use elysium_api::proto::{AuthUserRequest, AuthUserResponse};
use tonic::{Request, Response, Status};

pub async fn auth_user(
    state: ServerState,
    req: Request<AuthUserRequest>,
) -> Result<Response<AuthUserResponse>, Status> {
    let req = req.into_inner();
    let token = auth::create_token(state.database(), req.name, req.password).await?;

    Ok(Response::new(AuthUserResponse { token }))
}

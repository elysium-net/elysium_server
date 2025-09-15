use crate::api::user;
use crate::state::ServerState;
use elysium_api::proto::{CreateUserRequest, CreateUserResponse};
use elysium_api::secret::Secret;
use elysium_api::user::User;
use tonic::{Request, Response, Status};

pub async fn create_user(
    state: ServerState,
    req: Request<CreateUserRequest>,
) -> Result<Response<CreateUserResponse>, Status> {
    let req = req.into_inner();

    let email = state
        .email_verifier()
        .end_verify(req.email, req.email_token)
        .await?;

    user::create(
        state.database(),
        User::new(
            req.name,
            req.display,
            email,
            Secret::new(req.password),
            req.country_code as u32, // TODO: convert country code in .proto to u32
        ),
    )
    .await?;

    Ok(Response::new(CreateUserResponse {}))
}

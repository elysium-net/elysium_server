use crate::api::{auth, user};
use crate::state::ServerState;
use elysium_api::algo::post::COUNTRY_CODE_META_KEY;
use elysium_api::proto::{EditUserRequest, EditUserResponse};
use elysium_api::secret::Secret;
use elysium_api::user::User;
use tonic::{Request, Response, Status};

pub async fn edit_user(
    state: ServerState,
    req: Request<EditUserRequest>,
) -> Result<Response<EditUserResponse>, Status> {
    let user = auth::auth(state.database(), &req).await?;
    let req = req.into_inner();

    user::update(
        state.database(),
        User::new(
            user.name,
            req.display.unwrap_or(user.display),
            req.email.unwrap_or(user.email),
            if let Some(password) = req.password {
                Secret::new(password)
            } else {
                user.password
            },
            user.meta.get(COUNTRY_CODE_META_KEY.to_string()),
        ),
    )
    .await?;

    Ok(Response::new(EditUserResponse {}))
}

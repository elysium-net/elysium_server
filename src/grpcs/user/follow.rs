use crate::api::{auth, user};
use crate::state::ServerState;
use elysium_api::proto::{FollowUserRequest, FollowUserResponse};
use tonic::{Request, Response, Status};

pub async fn follow_user(
    state: ServerState,
    req: Request<FollowUserRequest>,
) -> Result<Response<FollowUserResponse>, Status> {
    let user = auth::auth(state.database(), &req).await?;

    let req = req.into_inner();

    if req.unfollow {
        user::unfollow(state.database(), user.name, req.user).await?;
    } else {
        user::follow(state.database(), user.name, req.user).await?;
    }

    Ok(Response::new(FollowUserResponse {}))
}

use crate::api::{auth, comment, post};
use crate::state::ServerState;
use elysium_api::algo::post::COUNTRY_CODE_META_KEY;
use elysium_api::post::Post;
use elysium_api::proto;
use elysium_api::proto::{CreatePostRequest, CreatePostResponse};
use tonic::{Request, Response, Status};

pub async fn create_post(
    state: ServerState,
    req: Request<CreatePostRequest>,
) -> Result<Response<CreatePostResponse>, Status> {
    let user = auth::auth(state.database(), &req).await?;

    let req = req.into_inner();

    let id = user.name.clone();
    let country_code = user.meta.get(COUNTRY_CODE_META_KEY.to_string());

    let post = post::create(
        state.database(),
        user.clone(),
        Post::new(
            proto::Post {
                content: req.contents,
                author: Some(user.profile()),
            },
            country_code,
            id,
        ),
    )
    .await?;

    if let Some(comment_on) = req.on {
        comment::on(state.database(), post.clone(), comment_on).await?;
    }

    Ok(Response::new(CreatePostResponse { post_id: post }))
}

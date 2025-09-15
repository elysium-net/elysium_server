use crate::grpcs;
use crate::state::ServerState;
use elysium_api::proto::elysium_server::Elysium;
use elysium_api::proto::{
    AuthUserRequest, AuthUserResponse, CreatePostRequest, CreatePostResponse, CreateUserRequest,
    CreateUserResponse, DeletePostRequest, DeletePostResponse, DeleteUserRequest,
    DeleteUserResponse, EditUserRequest, EditUserResponse, FollowUserRequest, FollowUserResponse,
    GetPostAuthorRequest, GetPostAuthorResponse, GetPostRequest, GetPostResponse,
    RecommendPostRequest, RecommendPostResponse, VerifyUserEmailRequest, VerifyUserEmailResponse,
};
use tonic::server::NamedService;
use tonic::{Request, Response, Status};

#[derive(Clone, Debug)]
pub struct ElysiumService {
    state: ServerState,
}

impl ElysiumService {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }

    pub fn state(&self) -> ServerState {
        self.state.clone()
    }
}

#[tonic::async_trait]
impl Elysium for ElysiumService {
    async fn get_post_author(
        &self,
        request: Request<GetPostAuthorRequest>,
    ) -> Result<Response<GetPostAuthorResponse>, Status> {
        grpcs::post::author::get_post_author(self.state(), request).await
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        grpcs::post::create::create_post(self.state(), request).await
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        grpcs::post::delete::delete_post(self.state(), request).await
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        grpcs::post::get::get_post(self.state(), request).await
    }

    async fn recommend_post(
        &self,
        request: Request<RecommendPostRequest>,
    ) -> Result<Response<RecommendPostResponse>, Status> {
        grpcs::post::recommend::recommend_post(self.state(), request).await
    }

    async fn auth_user(
        &self,
        request: Request<AuthUserRequest>,
    ) -> Result<Response<AuthUserResponse>, Status> {
        grpcs::user::auth::auth_user(self.state(), request).await
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        grpcs::user::create::create_user(self.state(), request).await
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        grpcs::user::delete::delete_user(self.state(), request).await
    }

    async fn edit_user(
        &self,
        request: Request<EditUserRequest>,
    ) -> Result<Response<EditUserResponse>, Status> {
        grpcs::user::edit::edit_user(self.state(), request).await
    }

    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, Status> {
        grpcs::user::follow::follow_user(self.state(), request).await
    }

    async fn verify_user_email(
        &self,
        request: Request<VerifyUserEmailRequest>,
    ) -> Result<Response<VerifyUserEmailResponse>, Status> {
        grpcs::user::verify::email::verify_email(self.state(), request).await
    }
}

impl NamedService for ElysiumService {
    const NAME: &'static str = "Elysium";
}

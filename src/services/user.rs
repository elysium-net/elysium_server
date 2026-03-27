use crate::error::Error;
use crate::state::ServerState;
use crate::{auth, user};
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::user::v1::user_service_server::UserService;
use elysium_rust::user::v1::{
    AuthUserRequest, AuthUserResponse, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetUserRequest, GetUserResponse, SearchUsersRequest, SearchUsersResponse,
    UpdateUserAvatarRequest, UpdateUserAvatarResponse, UpdateUserRequest, UpdateUserResponse, User,
    UserProfile, UserRole, auth_user_response, get_user_response,
};
use tonic::{Request, Response, Status};

pub struct Service {
    state: ServerState,
}

impl Service {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl UserService for Service {
    async fn auth_user(
        &self,
        request: Request<AuthUserRequest>,
    ) -> Result<Response<AuthUserResponse>, Status> {
        let AuthUserRequest { user_id, password } = request.into_inner();

        let resp = match auth::auth(self.state.database(), user_id, password).await {
            Ok(token) => AuthUserResponse {
                result: Some(auth_user_response::Result::Token(token)),
            },

            Err(err) => AuthUserResponse {
                result: Some(auth_user_response::Result::Error(err.into())),
            },
        };

        Ok(Response::new(resp))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let auth = auth::verify_role(self.state.database(), &request, UserRole::Admin).await;

        let mut user = request
            .into_inner()
            .user
            .ok_or(Status::invalid_argument("Request must contain a user"))?;

        user.password =
            auth::hash(user.password).map_err(|err| Status::internal(err.to_string()))?;

        let resp = if let Err(err) = auth {
            CreateUserResponse {
                error: Some(err.into()),
            }
        } else {
            match user::create(self.state.database(), user).await {
                Ok(_) => CreateUserResponse { error: None },
                Err(err) => CreateUserResponse {
                    error: Some(err.into()),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let auth = auth::verify_role(self.state.database(), &request, UserRole::Admin).await;

        let user = request.into_inner().user_id;

        let resp = if let Err(err) = auth {
            DeleteUserResponse {
                error: Some(err.into()),
            }
        } else {
            match user::delete(self.state.database(), &user).await {
                Ok(res) => match res {
                    None => DeleteUserResponse {
                        error: Some(Error::new(ErrorCode::NotFound, "User not found").into()),
                    },
                    Some(_) => DeleteUserResponse { error: None },
                },
                Err(err) => DeleteUserResponse {
                    error: Some(err.into()),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let auth = auth::verify_role(self.state.database(), &request, UserRole::Admin).await;
        let mut user = request
            .into_inner()
            .user
            .ok_or(Status::invalid_argument("Request must contain a user"))?;

        user.password =
            auth::hash(user.password).map_err(|err| Status::internal(err.to_string()))?;

        let resp = if let Err(err) = auth {
            UpdateUserResponse {
                error: Some(err.into()),
            }
        } else {
            match user::update(self.state.database(), user).await {
                Ok(_) => UpdateUserResponse { error: None },
                Err(err) => UpdateUserResponse {
                    error: Some(err.into()),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn update_user_avatar(
        &self,
        request: Request<UpdateUserAvatarRequest>,
    ) -> Result<Response<UpdateUserAvatarResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let UpdateUserAvatarRequest { user_id, avatar } = request.into_inner();
        let avatar = avatar.ok_or(Status::invalid_argument("Request must contain an avatar"))?;

        let resp = if let Err(err) = auth {
            UpdateUserAvatarResponse {
                error: Some(err.into()),
            }
        } else {
            match user::get(self.state.database(), &user_id).await {
                Ok(user) => match user {
                    Some(mut user) => {
                        user.icon = Some(avatar);
                        match user::update(self.state.database(), user).await {
                            Ok(_) => UpdateUserAvatarResponse { error: None },
                            Err(err) => UpdateUserAvatarResponse {
                                error: Some(err.into()),
                            },
                        }
                    }
                    None => UpdateUserAvatarResponse {
                        error: Some(Error::new(ErrorCode::NotFound, "User not found").into()),
                    },
                },
                Err(err) => UpdateUserAvatarResponse {
                    error: Some(err.into()),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let user = request.into_inner().user_id;

        let resp = if let Err(err) = auth {
            GetUserResponse {
                result: Some(get_user_response::Result::Error(err.into())),
            }
        } else {
            match user::get(self.state.database(), &user).await {
                Ok(user) => match user {
                    None => GetUserResponse {
                        result: Some(get_user_response::Result::Error(
                            Error::new(ErrorCode::NotFound, "User not found").into(),
                        )),
                    },

                    Some(user) => GetUserResponse {
                        result: Some(get_user_response::Result::User(user::to_profile(user))),
                    },
                },

                Err(err) => GetUserResponse {
                    result: Some(get_user_response::Result::Error(err.into())),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn search_users(
        &self,
        request: Request<SearchUsersRequest>,
    ) -> Result<Response<SearchUsersResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let query = request.into_inner().query;

        let resp = if let Err(err) = auth {
            SearchUsersResponse {
                users: Vec::new(),
                error: Some(err.into()),
            }
        } else {
            match user::search(self.state.database(), query).await {
                Ok(users) => SearchUsersResponse { users, error: None },
                Err(err) => SearchUsersResponse {
                    users: Vec::new(),
                    error: Some(err.into()),
                },
            }
        };

        Ok(Response::new(resp))
    }
}

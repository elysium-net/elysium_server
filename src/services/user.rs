use crate::error::Error;
use crate::state::ServerState;
use crate::{auth, user};
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::user::v1::user_service_server::UserService;
use elysium_rust::user::v1::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, SearchUsersRequest, SearchUsersResponse, UpdateUserAvatarRequest,
    UpdateUserAvatarResponse, UpdateUserRequest, UpdateUserResponse, UserRole, get_user_response,
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
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let auth = auth::verify_role(self.state.database(), &request, UserRole::Admin).await;

        let user = request
            .into_inner()
            .user
            .ok_or(Status::invalid_argument("Request must contain a user"))?;

        let resp = {
            if let Err(err) = auth {
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

        let resp = {
            if let Err(err) = auth {
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
            }
        };

        Ok(Response::new(resp))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        todo!()
    }

    async fn update_user_avatar(
        &self,
        request: Request<UpdateUserAvatarRequest>,
    ) -> Result<Response<UpdateUserAvatarResponse>, Status> {
        todo!()
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let user = request.into_inner().user_id;

        let resp = {
            if let Err(err) = auth {
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
            }
        };

        Ok(Response::new(resp))
    }

    async fn search_users(
        &self,
        request: Request<SearchUsersRequest>,
    ) -> Result<Response<SearchUsersResponse>, Status> {
        todo!()
    }
}

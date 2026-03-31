use crate::services::user;
use crate::tests;
use elysium_rust::user::v1::user_service_server::UserService;
use elysium_rust::user::v1::{
    AuthUserRequest, CreateUserRequest, DeleteUserRequest, GetUserRequest, SearchUsersRequest,
    UpdateUserRequest, User, UserRole, auth_user_response, get_user_response,
};
use tonic::Request;

#[tokio::test]
async fn user() {
    let (state, token) = tests::init().await;
    let service = user::Service::new(state);

    let mut user = User {
        user_id: "foobar".to_string(),
        username: "Foo Bar".to_string(),
        email: "foobar".to_string(),
        password: "foobar".to_string(),
        role: UserRole::Supervisor as i32,
        icon: None,
    };

    tracing::info!("Creating user...");
    service
        .create_user(tests::request(
            CreateUserRequest {
                user: Some(user.clone()),
            },
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .error
        .map(|err| panic!("{err:?}"));

    tracing::info!("Updating user...");
    user.username = "Foo Bar Baz".to_string();
    service
        .update_user(tests::request(
            UpdateUserRequest {
                user: Some(user.clone()),
            },
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .error
        .map(|err| panic!("{err:?}"));

    tracing::info!("Authenticating as user...");
    let user_token = match service
        .auth_user(Request::new(AuthUserRequest {
            user_id: user.user_id.clone(),
            password: user.password.clone(),
        }))
        .await
        .unwrap()
        .into_inner()
        .result
        .unwrap()
    {
        auth_user_response::Result::Token(token) => token,
        auth_user_response::Result::Error(err) => panic!("{err:?}"),
    };

    tracing::info!("Getting user...");
    match service
        .get_user(tests::request(
            GetUserRequest {
                user_id: user.user_id.clone(),
            },
            [("Authorization", user_token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .result
        .unwrap()
    {
        get_user_response::Result::User(got_user) => {
            assert_eq!(got_user, crate::user::to_profile(user.clone()));
        }
        get_user_response::Result::Error(err) => panic!("{err:?}"),
    }

    tracing::info!("Searching user...");
    let result = service
        .search_users(tests::request(
            SearchUsersRequest {
                query: "Foo".to_string(),
            },
            [("Authorization", user_token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner();

    result.error.map(|err| panic!("{err:?}"));
    assert_eq!(result.users[0], crate::user::to_profile(user.clone()));

    tracing::info!("Deleting user...");
    service
        .delete_user(tests::request(
            DeleteUserRequest {
                user_id: user.user_id,
            },
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .error
        .map(|err| panic!("{err:?}"));
}

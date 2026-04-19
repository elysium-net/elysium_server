use crate::config;
use crate::state::ServerState;
use crate::utils::RESOURCE_CHUNK_SIZE;
use elysium_rust::general::v1::general_service_server::GeneralService;
use elysium_rust::general::v1::{
    ClearStateRequest, ClearStateResponse, GetConfigRequest, GetConfigResponse,
};
use tonic::{Request, Response, Status};

#[cfg(feature = "testing")]
const TEST_NEW_USER_NAME: &str = "user";
#[cfg(feature = "testing")]
const TEST_NEW_USER_PASS: &str = "user";

#[cfg(feature = "testing")]
const TEST_SUPERVISOR_NAME: &str = "supervisor";
#[cfg(feature = "testing")]
const TEST_SUPERVISOR_PASS: &str = "supervisor";

#[cfg(feature = "testing")]
const TEST_ADMIN_NAME: &str = "admin";
#[cfg(feature = "testing")]
const TEST_ADMIN_PASS: &str = "admin";

pub struct Service {
    #[allow(unused)]
    state: ServerState,
}

impl Service {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl GeneralService for Service {
    async fn get_config(
        &self,
        _: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        let config = config::get();

        Ok(Response::new(GetConfigResponse {
            resource_chunk_size: RESOURCE_CHUNK_SIZE as u32,
            allow_message_delete: config.service_allow_message_delete,
            allow_message_update: config.service_allow_message_update,
        }))
    }

    async fn clear_state(
        &self,
        _: Request<ClearStateRequest>,
    ) -> Result<Response<ClearStateResponse>, Status> {
        #[cfg(feature = "testing")]
        {
            clear_state(&self.state).await;

            Ok(Response::new(ClearStateResponse {}))
        }

        #[cfg(not(feature = "testing"))]
        Err(Status::failed_precondition("Server not in testing mode"))
    }
}

#[cfg(feature = "testing")]
async fn clear_state(state: &ServerState) {
    let database = state.database();

    tracing::info!("Detected test environment. Clearing database...");

    database
        .query(
            r#"
REMOVE TABLE user;
REMOVE TABLE channel;
REMOVE TABLE message;"#,
        )
        .await
        .expect("Failed to drop user table");

    // Setup database again, since we just cleared all tables
    database.setup().await;

    tracing::info!("Creating test user with role user...");
    crate::user::create(
        database,
        elysium_rust::User {
            user_id: TEST_NEW_USER_NAME.to_string(),
            username: TEST_NEW_USER_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: crate::auth::hash(TEST_NEW_USER_PASS.to_string())
                .expect("Failed to hash new user password"),
            role: elysium_rust::user::v1::UserRole::UserUnspecified as i32,
            icon: crate::user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new test user");

    tracing::info!("Creating test user with role supervisor...");
    crate::user::create(
        database,
        elysium_rust::User {
            user_id: TEST_SUPERVISOR_NAME.to_string(),
            username: TEST_SUPERVISOR_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: crate::auth::hash(TEST_SUPERVISOR_PASS.to_string())
                .expect("Failed to hash supervisor password"),
            role: elysium_rust::user::v1::UserRole::Supervisor as i32,
            icon: crate::user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new admin test user");

    tracing::info!("Creating test user with role admin...");
    crate::user::create(
        database,
        elysium_rust::User {
            user_id: TEST_ADMIN_NAME.to_string(),
            username: TEST_ADMIN_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: crate::auth::hash(TEST_ADMIN_PASS.to_string())
                .expect("Failed to hash admin password"),
            role: elysium_rust::user::v1::UserRole::Admin as i32,
            icon: crate::user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new supervisor test user");
}

use crate::state::ServerState;
use crate::user;
use elysium_rust::User;
use elysium_rust::user::v1::UserRole;

const NEW_USER_NAME: &str = "user";
const NEW_USER_PASS: &str = "user";

const SUPERVISOR_NAME: &str = "supervisor";
const SUPERVISOR_PASS: &str = "supervisor";

const ADMIN_NAME: &str = "admin";
const ADMIN_PASS: &str = "admin";

pub async fn init(state: &ServerState) {
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
    user::create(
        database,
        User {
            user_id: NEW_USER_NAME.to_string(),
            username: NEW_USER_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: NEW_USER_PASS.to_string(),
            role: UserRole::UserUnspecified as i32,
            icon: user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new test user");

    tracing::info!("Creating test user with role supervisor...");
    user::create(
        database,
        User {
            user_id: SUPERVISOR_NAME.to_string(),
            username: SUPERVISOR_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: SUPERVISOR_PASS.to_string(),
            role: UserRole::Supervisor as i32,
            icon: user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new admin test user");

    tracing::info!("Creating test user with role admin...");
    user::create(
        database,
        User {
            user_id: ADMIN_NAME.to_string(),
            username: ADMIN_NAME.to_string(),
            email: "foo@bar.baz".to_string(),
            password: ADMIN_PASS.to_string(),
            role: UserRole::Admin as i32,
            icon: user::default_icon(),
        },
    )
    .await
    .expect("Failed to create new supervisor test user");
}

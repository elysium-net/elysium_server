use crate::state::ServerState;
use crate::{auth, config, trace};
use std::sync::atomic::{AtomicBool, Ordering};
use tonic::Request;
use tonic::metadata::MetadataKey;

mod chat;
mod user;

fn request<T>(
    message: T,
    metadata: impl IntoIterator<Item = (impl ToString, impl ToString)>,
) -> Request<T> {
    let mut req = Request::new(message);

    for (key, val) in metadata {
        let key = key.to_string();
        let val = val.to_string();
        req.metadata_mut()
            .append(key.parse::<MetadataKey<_>>().unwrap(), val.parse().unwrap());
    }

    req
}

async fn init() -> (ServerState, String) {
    static INIT: AtomicBool = AtomicBool::new(false);

    if !INIT.swap(true, Ordering::SeqCst) {
        config::init();
        trace::init_logger();
        auth::init().await;
    }

    let state = ServerState::new().await;

    crate::user::create_admin(state.database())
        .await
        .expect("Failed to create admin user");

    let token = auth::auth(state.database(), "admin".to_string(), "admin".to_string())
        .await
        .unwrap();

    (state, token)
}

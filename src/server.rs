use crate::config;
use crate::service::ElysiumService;
use crate::state::ServerState;
use elysium_api::proto::elysium_server::ElysiumServer;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::time::Duration;
use tonic::transport::Server;
use tonic_middleware::RequestInterceptorLayer;
use tonic_web::GrpcWebLayer;

pub async fn launch() {
    let addr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::from_str(config::server::HOST.as_str()).unwrap(),
        *config::server::PORT,
    ));

    tracing::info!("Building Server State...");

    let state = ServerState::new().await.expect("failed to create state");

    tracing::info!("Starting cleanup routine...");

    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(
                *config::runtime::CLEANUP_PERIOD_MILLIS,
            ))
            .await;

            cleanup(&state_clone).await;
        }
    });

    tracing::info!("Launching gRPC server on http://{}", addr);

    Server::builder()
        .accept_http1(true)
        .timeout(Duration::from_secs(*config::server::TIMEOUT_SECS))
        .layer(GrpcWebLayer::new())
        .serve_with_shutdown(
            addr,
            ElysiumServer::new(ElysiumService::new(state)),
            shutdown_signal(),
        )
        .await
        .expect("failed to start tonic server");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to register ctrl+c handler");

    tracing::info!("Server stopped. Goodbye!");
}

async fn cleanup(state: &ServerState) {
    state.email_verifier().cleanup().await;
}

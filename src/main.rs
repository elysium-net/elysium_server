use crate::services::{ChatService, ResourceService, UserService};
use crate::state::ServerState;
use elysium_rust::chat::v1::chat_service_server::ChatServiceServer;
use elysium_rust::resource::v1::resource_service_server::ResourceServiceServer;
use elysium_rust::user::v1::user_service_server::UserServiceServer;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tonic::transport::Server;

mod auth;
mod chat;
mod config;
mod database;
mod error;
mod resource;
mod services;
mod state;
mod trace;
mod user;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    println!("Loading configuration...");
    config::init();
    let config = config::get();

    trace::init_logger();
    tracing::info!("Logger initialized!");

    tracing::info!("########## CONFIGURATION START ##########");
    println!("{config:#?}");
    tracing::info!("########### CONFIGURATION END ###########");

    tracing::info!("Initializing runtime...");
    tokio::runtime::Builder::new_multi_thread()
        .enable_alt_timer()
        .enable_io()
        .max_io_events_per_tick(config.rt_max_io_events_per_tick)
        .thread_keep_alive(Duration::from_secs(config.rt_thread_keep_alive))
        .global_queue_interval(config.rt_global_queue_interval)
        .event_interval(config.rt_event_interval)
        .worker_threads(config.rt_worker_threads)
        .max_blocking_threads(config.rt_max_blocking_threads)
        .thread_name("elysium-worker")
        .build()
        .expect("Failed to build tokio runtime")
        .block_on(async {
            tracing::info!("Initializing authentication...");
            auth::init().await;

            tokio::select! {
                _ = serve() => (),
                _ = exit_signal() => (),
            }
        });
}

async fn serve() {
    let config = config::get();

    let addr = SocketAddr::from_str(config.net_address.as_str()).expect("Failed to parse address");

    tracing::info!("Initializing Server State...");
    let state = ServerState::new().await;

    // Create initial admin if not present
    user::create_admin(state.database())
        .await
        .expect("Failed to create admin user");

    tracing::info!("Creating reflection server...");
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(elysium_rust::FILE_DESCRIPTOR_SET)
        .include_reflection_service(true)
        .build_v1alpha()
        .expect("Failed to build reflection server");

    tracing::info!("Serving Elysium at '{}'...", config.net_address.as_str());
    Server::builder()
        .add_service(reflection)
        .add_service(UserServiceServer::new(UserService::new(state.clone())))
        .add_service(ChatServiceServer::new(ChatService::new(state.clone())))
        .add_service(ResourceServiceServer::new(ResourceService::new(state)))
        .serve(addr)
        .await
        .expect("Failed to serve elysium service");
}

async fn exit_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to get Ctrl+C signal");

    tracing::info!("Shutting down elysium...");
}

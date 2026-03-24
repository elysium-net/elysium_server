use crate::services::{ChatService, ResourceService, UserService};
use crate::state::ServerState;
use elysium_rust::chat::v1::chat_service_server::ChatServiceServer;
use elysium_rust::resource::v1::resource_service_server::ResourceServiceServer;
use elysium_rust::user::v1::user_service_server::UserServiceServer;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio::task::JoinSet;
use tonic::transport::Server;
use tracing::metadata::LevelFilter;

mod auth;
mod cfg;
mod database;
mod error;
mod services;
mod state;
mod user;

fn main() {
    init_logger();

    tracing::info!("Tracing logger initialized!");

    tracing::info!("############### ELYSIUM CONFIG START ###############");
    tracing::info!("MAX_IO_EVENTS_PER_TICK: {}", *cfg::MAX_IO_EVENTS_PER_TICK);
    tracing::info!("THREAD_KEEP_ALIVE: {}", *cfg::THREAD_KEEP_ALIVE);
    tracing::info!("GLOBAL_QUEUE_INTERVAL: {}", *cfg::GLOBAL_QUEUE_INTERVAL);
    tracing::info!("EVENT_INTERVAL: {}", *cfg::EVENT_INTERVAL);
    tracing::info!("WORKER_THREADS: {}", *cfg::WORKER_THREADS);
    tracing::info!("MAX_BLOCKING_THREADS: {}", *cfg::MAX_BLOCKING_THREADS);
    tracing::info!("DATABASE_ADDRESS: {}", cfg::DATABASE_ADDRESS.as_str());
    tracing::info!("LOG_FILE_NAMES: {}", *cfg::LOG_FILE_NAMES);
    tracing::info!("LOG_TARGETS: {}", *cfg::LOG_TARGETS);
    tracing::info!("LOG_LEVEL: {}", cfg::LOG_LEVEL.as_str());
    tracing::info!("LOG_THREADS: {}", *cfg::LOG_THREADS);
    tracing::info!("LOG_TIME: {}", *cfg::LOG_TIME);
    tracing::info!("ADDRESS: {}", cfg::ADDRESS.as_str());
    tracing::info!("PUBLIC_AUTH_KEY: {}", cfg::PUBLIC_AUTH_KEY.as_str());
    tracing::info!("PRIVATE_AUTH_KEY: {}", cfg::PRIVATE_AUTH_KEY.as_str());
    tracing::info!("################ ELYSIUM CONFIG END ################");

    tracing::info!("Initializing runtime...");
    tokio::runtime::Builder::new_multi_thread()
        .enable_alt_timer()
        .enable_io()
        .max_io_events_per_tick(*cfg::MAX_IO_EVENTS_PER_TICK)
        .thread_keep_alive(Duration::from_secs(*cfg::THREAD_KEEP_ALIVE))
        .global_queue_interval(*cfg::GLOBAL_QUEUE_INTERVAL)
        .event_interval(*cfg::EVENT_INTERVAL)
        .worker_threads(*cfg::WORKER_THREADS)
        .max_blocking_threads(*cfg::MAX_BLOCKING_THREADS)
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
    let addr = SocketAddr::from_str(cfg::ADDRESS.as_str()).expect("Failed to parse address");

    tracing::info!("Initializing Server State...");
    let state = ServerState::new().await;

    tracing::info!("Serving Elysium at '{}'...", cfg::ADDRESS.as_str());
    Server::builder()
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

fn init_logger() {
    let mut builder = tracing_subscriber::FmtSubscriber::builder()
        .with_file(*cfg::LOG_FILE_NAMES)
        .with_target(*cfg::LOG_TARGETS)
        .with_thread_names(*cfg::LOG_THREADS)
        .with_thread_ids(*cfg::LOG_THREADS)
        .with_max_level(LevelFilter::from_str(cfg::LOG_LEVEL.as_str()).expect("Invalid log level"));

    if !*cfg::LOG_TIME {
        builder.without_time().init();
    } else {
        builder.init();
    }
}

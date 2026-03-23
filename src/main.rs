use crate::services::{ChatService, ResourceService, UserService};
use crate::state::ServerState;
use elysium_rust::chat::v1::chat_service_server::ChatServiceServer;
use elysium_rust::resource::v1::resource_service_server::ResourceServiceServer;
use elysium_rust::user::v1::user_service_server::UserServiceServer;
use fastrace::collector::{Config, ConsoleReporter};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tonic::transport::Server;

mod auth;
mod cfg;
mod database;
mod error;
mod services;
mod state;
mod user;

fn main() {
    init_logger();

    log::info!("Tracing logger initialized!");

    log::info!("########## ELYSIUM CONFIG ##########");
    log::info!("MAX_IO_EVENTS_PER_TICK: {:?}", cfg::MAX_IO_EVENTS_PER_TICK);
    log::info!("THREAD_KEEP_ALIVE: {:?}", cfg::THREAD_KEEP_ALIVE);
    log::info!("GLOBAL_QUEUE_INTERVAL: {:?}", cfg::GLOBAL_QUEUE_INTERVAL);
    log::info!("EVENT_INTERVAL: {:?}", cfg::EVENT_INTERVAL);
    log::info!("WORKER_THREADS: {:?}", cfg::WORKER_THREADS);
    log::info!("MAX_BLOCKING_THREADS: {:?}", cfg::MAX_BLOCKING_THREADS);
    log::info!("DATABASE_ADDRESS: {:?}", cfg::DATABASE_ADDRESS);
    log::info!("LOG_FILE_NAMES: {:?}", cfg::LOG_FILE_NAMES);
    log::info!("LOG_TARGETS: {:?}", cfg::LOG_TARGETS);
    log::info!("LOG_LEVEL: {:?}", cfg::LOG_LEVEL);
    log::info!("LOG_THREADS: {:?}", cfg::LOG_THREADS);
    log::info!("LOG_TIME: {:?}", cfg::LOG_TIME);
    log::info!("MAX_ENCODING_MSG_SIZE: {:?}", cfg::MAX_ENCODING_MSG_SIZE);
    log::info!("MAX_DECODING_MSG_SIZE: {:?}", cfg::MAX_DECODING_MSG_SIZE);
    log::info!("ADDRESS: {:?}", cfg::ADDRESS);
    log::info!(
        "PUBLIC_AUTH_KEY: {:?}",
        cfg::PUBLIC_AUTH_KEY.replace(|c| true, "*")
    );
    log::info!(
        "PRIVATE_AUTH_KEY: {:?}",
        cfg::PRIVATE_AUTH_KEY.replace(|c| true, "*")
    );

    log::info!("Initializing authentication...");
    auth::init();

    log::info!("Initializing runtime...");
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
            serve().await;
        });

    fastrace::flush();
}

async fn serve() {
    let addr = SocketAddr::from_str(cfg::ADDRESS.as_str()).expect("Failed to parse address");

    log::info!("Initializing Server State...");
    let state = ServerState::new().await;

    log::info!("Launching Server...");
    Server::builder()
        .add_service(UserServiceServer::new(UserService::new(state.clone())))
        .add_service(ChatServiceServer::new(ChatService::new(state.clone())))
        .add_service(ResourceServiceServer::new(ResourceService::new(state)))
        .serve(addr)
        .await
        .expect("Failed to serve elysium service");
}

fn init_logger() {
    fastrace::set_reporter(ConsoleReporter, Config::default());
}

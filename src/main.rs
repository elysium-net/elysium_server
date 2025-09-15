use std::str::FromStr;
use tracing::level_filters::LevelFilter;

pub mod api;
pub mod config;
pub mod database;
pub mod email;
pub mod error;
pub mod grpcs;
pub mod server;
pub mod service;
pub mod state;
pub mod templates;

fn main() {
    init_logger();

    tracing::info!("Logger initialized!");

    tracing::info!("Listing configuration...");
    config_info();

    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(*config::runtime::RT_WORKERS)
        .thread_stack_size(*config::runtime::RT_STACK_SIZE)
        .global_queue_interval(*config::runtime::RT_GLOBAL_QUEUE_INTERVAL)
        .event_interval(*config::runtime::RT_EVENT_INTERVAL)
        .max_blocking_threads(*config::runtime::RT_MAX_BLOCKING_THREADS)
        .max_io_events_per_tick(*config::runtime::RT_MAX_IO_EVENTS_PER_TICK)
        .thread_name("tokio-worker")
        .build()
        .expect("failed to build tokio runtime")
        .block_on(server::launch());
}

fn init_logger() {
    let logger = tracing_subscriber::fmt()
        .with_thread_ids(*config::logger::LOG_THREAD_IDS)
        .with_max_level(
            LevelFilter::from_str(config::logger::LOG_LEVEL.as_str())
                .expect("failed to parse log level"),
        )
        .with_file(*config::logger::LOG_FILES)
        .with_ansi(*config::logger::LOG_ANSI)
        .with_thread_names(*config::logger::LOG_THREAD_NAMES)
        .with_line_number(*config::logger::LOG_LINE_NUMBER)
        .with_target(*config::logger::LOG_TARGET);

    if !*config::logger::LOG_TIMESTAMP {
        logger.without_time().init();
    } else {
        logger.init();
    }
}

fn config_info() {
    tracing::info_span!("configuration").in_scope(|| {
        tracing::info!("########## CONFIGURATION BEGIN ##########");

        tracing::info_span!("app").in_scope(|| {
            tracing::info!("JWT_ALGO: {}", config::app::JWT_ALGO.as_str());
            tracing::info!("JWT_KEY: ***");
            tracing::info!("JWT_EXPIRY_SECS: {}", *config::app::JWT_EXPIRY_SECS);
            tracing::info!("DOMAIN: {}", config::app::DOMAIN.as_str());
            tracing::info!(
                "RECOMMENDED_POSTS_LEN: {}",
                *config::app::RECOMMENDED_POSTS_LEN
            );
        });

        tracing::info_span!("email").in_scope(|| {
            tracing::info!(
                "EMAIL_SMTP_HOST: {}",
                config::email::EMAIL_SMTP_HOST.as_str()
            );
            tracing::info!("EMAIL_SMTP_PORT: {}", *config::email::EMAIL_SMTP_PORT);
            tracing::info!(
                "EMAIL_SMTP_USER: {}",
                config::email::EMAIL_SMTP_USER.as_str()
            );
            tracing::info!("EMAIL_SMTP_PASSWORD: ***");
            tracing::info!(
                "EMAIL_VERIFY_TOKEN_LEN: {}",
                *config::email::EMAIL_VERIFY_TOKEN_LEN
            );
            tracing::info!(
                "EMAIL_VERIFY_EXPIRY_SECS: {}",
                *config::email::EMAIL_VERIFY_EXPIRY_SECS
            );
        });

        tracing::info_span!("server").in_scope(|| {
            tracing::info!("EARLY_EXIT: {}", *config::server::EARLY_EXIT);
            tracing::info!("HOST: {}", config::server::HOST.as_str());
            tracing::info!("PORT: {}", *config::server::PORT);
            tracing::info!("SERVER_TIMEOUT_SECS: {}", *config::server::TIMEOUT_SECS);
        });

        tracing::info_span!("logger").in_scope(|| {
            tracing::info!("LOG_LEVEL: {}", config::logger::LOG_LEVEL.as_str());
            tracing::info!("LOG_THREAD_IDS: {}", *config::logger::LOG_THREAD_IDS);
            tracing::info!("LOG_FILES: {}", *config::logger::LOG_FILES);
            tracing::info!("LOG_ANSI: {}", *config::logger::LOG_ANSI);
            tracing::info!("LOG_THREAD_NAMES: {}", *config::logger::LOG_THREAD_NAMES);
            tracing::info!("LOG_TARGET: {}", *config::logger::LOG_TARGET);
            tracing::info!("LOG_LINE_NUMBER: {}", *config::logger::LOG_LINE_NUMBER);
            tracing::info!("LOG_TIMESTAMP: {}", *config::logger::LOG_TIMESTAMP);
        });

        tracing::info_span!("runtime").in_scope(|| {
            tracing::info!("RT_WORKERS: {}", *config::runtime::RT_WORKERS);
            tracing::info!("RT_STACK_SIZE: {}", *config::runtime::RT_STACK_SIZE);
            tracing::info!(
                "RT_GLOBAL_QUEUE_INTERVAL: {}",
                *config::runtime::RT_GLOBAL_QUEUE_INTERVAL
            );
            tracing::info!("RT_EVENT_INTERVAL: {}", *config::runtime::RT_EVENT_INTERVAL);
            tracing::info!(
                "RT_MAX_IO_EVENTS_PER_TICK: {}",
                *config::runtime::RT_MAX_IO_EVENTS_PER_TICK
            );
            tracing::info!(
                "RT_MAX_BLOCKING_THREADS: {}",
                *config::runtime::RT_MAX_BLOCKING_THREADS
            );
            tracing::info!(
                "CLEANUP_PERIOD_MILLIS: {}",
                *config::runtime::CLEANUP_PERIOD_MILLIS
            );
        });

        tracing::info_span!("database").in_scope(|| {
            tracing::info!("DB_URL: {}", config::database::DB_URL.as_str());
            tracing::info!("DB_DATABASE: {}", config::database::DB_DATABASE.as_str());
            tracing::info!("DB_NAMESPACE: {}", config::database::DB_NAMESPACE.as_str());
            tracing::info!("DB_USER: {}", config::database::DB_USER.as_str());
            tracing::info!("DB_PASS: ***");
            tracing::info!(
                "DB_SESSION_EXPIRY_SECS: {}",
                *config::database::DB_SESSION_EXPIRY_SECS
            );
        });

        tracing::info!("########## CONFIGURATION END ##########");
    });
}

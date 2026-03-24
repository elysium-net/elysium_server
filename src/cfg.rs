use std::sync::LazyLock;

macro_rules! env_or_default {
    ($key:literal, $default:expr) => {
        LazyLock::new(|| {
            std::env::var($key)
                .map(|v| v.parse().expect("Failed to parse environment variable"))
                .unwrap_or_else(|_| $default)
        })
    };
}

pub static ADDRESS: LazyLock<String> =
    env_or_default!("ELY_ADDRESS", "127.0.0.1:50051".to_string());

pub static MAX_IO_EVENTS_PER_TICK: LazyLock<usize> =
    env_or_default!("ELY_MAX_IO_EVENTS_PER_TICK", 1024);

pub static THREAD_KEEP_ALIVE: LazyLock<u64> = env_or_default!("ELY_THREAD_KEEP_ALIVE", 10);

pub static GLOBAL_QUEUE_INTERVAL: LazyLock<u32> = env_or_default!("ELY_GLOBAL_QUEUE_INTERVAL", 31);

pub static EVENT_INTERVAL: LazyLock<u32> = env_or_default!("ELY_EVENT_INTERVAL", 61);

pub static WORKER_THREADS: LazyLock<usize> = env_or_default!("ELY_WORKER_THREADS", 4);

pub static MAX_BLOCKING_THREADS: LazyLock<usize> = env_or_default!("ELY_MAX_BLOCKING_THREADS", 256);

pub static DATABASE_ADDRESS: LazyLock<String> =
    env_or_default!("ELY_DATABASE_ADDRESS", "127.0.0.1:8000".to_string());

pub static DATABASE_USER: LazyLock<String> =
    env_or_default!("ELY_DATABASE_USER", "root".to_string());

pub static DATABASE_PASSWORD: LazyLock<String> =
    env_or_default!("ELY_DATABASE_PASSWORD", "root".to_string());

pub static DATABASE_NAMESPACE: LazyLock<String> =
    env_or_default!("ELY_DATABASE_NAMESPACE", "elysium".to_string());

pub static DATABASE_NAME: LazyLock<String> =
    env_or_default!("ELY_DATABASE_NAME", "database".to_string());

pub static LOG_FILE_NAMES: LazyLock<bool> = env_or_default!("ELY_LOG_FILE_NAMES", false);

pub static LOG_TARGETS: LazyLock<bool> = env_or_default!("ELY_LOG_TARGETS", false);

pub static LOG_LEVEL: LazyLock<String> = env_or_default!("ELY_LOG_LEVEL", "info".to_string());

pub static LOG_THREADS: LazyLock<bool> = env_or_default!("ELY_LOG_THREADS", false);

pub static LOG_TIME: LazyLock<bool> = env_or_default!("ELY_LOG_TIME", false);

pub static PUBLIC_AUTH_KEY: LazyLock<String> = env_or_default!(
    "ELY_PUBLIC_AUTH_KEY",
    "./dummy-data/public-key.pem".to_string()
);

pub static PRIVATE_AUTH_KEY: LazyLock<String> = env_or_default!(
    "ELY_PRIVATE_AUTH_KEY",
    "./dummy-data/private-key.pem".to_string()
);

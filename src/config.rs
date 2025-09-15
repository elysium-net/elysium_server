macro_rules! env_or_default {
    ($name:literal, $default:expr) => {
        std::sync::LazyLock::new(|| {
            std::env::var($name)
                .map(|s| s.parse().expect("failed to parse env var value"))
                .unwrap_or($default)
        })
    };
}

pub mod app {
    use std::sync::LazyLock;

    pub static JWT_ALGO: LazyLock<String> = env_or_default!("JWT_ALGO", "HS256".to_string());
    pub static JWT_KEY: LazyLock<String> = env_or_default!(
        "JWT_KEY",
        "a-string-secret-at-least-256-bits-long".to_string()
    );
    pub static JWT_EXPIRY_SECS: LazyLock<u64> =
        env_or_default!("JWT_EXPIRY_SECS", 60 * 60 * 24 * 14);

    pub static DOMAIN: LazyLock<String> = env_or_default!("DOMAIN", "localhost".to_string());

    pub static RECOMMENDED_POSTS_LEN: LazyLock<usize> =
        env_or_default!("RECOMMENDED_POSTS_LEN", 10);
}

pub mod email {
    use std::sync::LazyLock;

    pub static EMAIL_SMTP_HOST: LazyLock<String> =
        env_or_default!("EMAIL_SMTP_HOST", "".to_string());
    pub static EMAIL_SMTP_PORT: LazyLock<u16> = env_or_default!("EMAIL_SMTP_PORT", 25);
    pub static EMAIL_SMTP_USER: LazyLock<String> =
        env_or_default!("EMAIL_SMTP_USER", "".to_string());
    pub static EMAIL_SMTP_PASSWORD: LazyLock<String> =
        env_or_default!("EMAIL_SMTP_PASSWORD", "".to_string());

    pub static EMAIL_VERIFY_TOKEN_LEN: LazyLock<usize> =
        env_or_default!("EMAIL_VERIFY_TOKEN_LEN", 6);
    pub static EMAIL_VERIFY_EXPIRY_SECS: LazyLock<u64> =
        env_or_default!("EMAIL_VERIFY_EXPIRY_SECS", 60 * 5);
}

pub mod server {
    use std::sync::LazyLock;

    pub static EARLY_EXIT: LazyLock<bool> = env_or_default!("EARLY_EXIT", false);
    pub static HOST: LazyLock<String> = env_or_default!("HOST", "127.0.0.1".to_string());
    pub static PORT: LazyLock<u16> = env_or_default!("PORT", 50051);
    pub static TIMEOUT_SECS: LazyLock<u64> = env_or_default!("SERVER_TIMEOUT_SECS", 10);
}

pub mod logger {
    use std::sync::LazyLock;

    pub static LOG_LEVEL: LazyLock<String> = env_or_default!("LOG_LEVEL", "INFO".to_string());
    pub static LOG_THREAD_IDS: LazyLock<bool> = env_or_default!("LOG_THREAD_IDS", false);
    pub static LOG_FILES: LazyLock<bool> = env_or_default!("LOG_FILES", true);
    pub static LOG_ANSI: LazyLock<bool> = env_or_default!("LOG_ANSI", true);
    pub static LOG_THREAD_NAMES: LazyLock<bool> = env_or_default!("LOG_THREAD_NAMES", false);
    pub static LOG_TARGET: LazyLock<bool> = env_or_default!("LOG_TARGETS", false);
    pub static LOG_LINE_NUMBER: LazyLock<bool> = env_or_default!("LOG_LINE_NUMBER", true);
    pub static LOG_TIMESTAMP: LazyLock<bool> = env_or_default!("LOG_TIMESTAMP", true);
}

pub mod runtime {
    use std::sync::LazyLock;

    pub static RT_WORKERS: LazyLock<usize> = env_or_default!("RT_WORKERS", 4);
    pub static RT_STACK_SIZE: LazyLock<usize> = env_or_default!("RT_STACK_SIZE", 1024 * 1024 * 2);
    pub static RT_GLOBAL_QUEUE_INTERVAL: LazyLock<u32> =
        env_or_default!("RT_GLOBAL_QUEUE_INTERVAL", 31);
    pub static RT_EVENT_INTERVAL: LazyLock<u32> = env_or_default!("RT_EVENT_INTERVAL", 31);
    pub static RT_MAX_IO_EVENTS_PER_TICK: LazyLock<usize> =
        env_or_default!("RT_MAX_IO_EVENTS_PER_TICK", 1024);
    pub static RT_MAX_BLOCKING_THREADS: LazyLock<usize> =
        env_or_default!("RT_MAX_BLOCKING_THREADS", 512);

    pub static CLEANUP_PERIOD_MILLIS: LazyLock<u64> =
        env_or_default!("CLEANUP_PERIOD_MILLIS", 1000);
}

pub mod database {
    use std::sync::LazyLock;

    pub static DB_URL: LazyLock<String> = env_or_default!("DB_URL", "127.0.0.1:8000".to_string());
    pub static DB_DATABASE: LazyLock<String> =
        env_or_default!("DB_DATABASE", "elysium".to_string());
    pub static DB_NAMESPACE: LazyLock<String> =
        env_or_default!("DB_NAMESPACE", "elysium".to_string());
    pub static DB_USER: LazyLock<String> = env_or_default!("DB_USER", "root".to_string());
    pub static DB_PASS: LazyLock<String> = env_or_default!("DB_PASS", "root".to_string());

    pub static DB_SESSION_EXPIRY_SECS: LazyLock<u64> =
        env_or_default!("DB_SESSION_EXPIRY_SECS", 60 * 60 * 24 * 14);
}

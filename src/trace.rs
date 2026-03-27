use crate::cfg;
use std::str::FromStr;
use tracing::level_filters::LevelFilter;

pub fn init_logger() {
    let builder = tracing_subscriber::FmtSubscriber::builder()
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

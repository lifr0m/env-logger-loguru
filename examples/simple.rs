fn main() {
    env_logger::Builder::new()
        .format(env_logger_loguru::format)
        .filter_level(log::LevelFilter::Trace)
        .init();

    log::error!("hello");
    log::warn!("hello");
    log::info!("hello");
    log::debug!("hello");
    log::trace!("hello");
}

mod level;

use chrono::prelude::*;
use colored::Colorize;
pub use level::Level;

pub fn log(level: Level, message: &str) {
    eprintln!(
        "{} | {:<8} - {}",
        Local::now().format("%F %T%.3f").to_string().green(),
        level.colorize(&level.to_string()),
        level.colorize(message),
    );
}

pub fn critical(message: &str) {
    log(Level::Critical, message);
}

pub fn error(message: &str) {
    log(Level::Error, message);
}

pub fn warning(message: &str) {
    log(Level::Warning, message);
}

pub fn success(message: &str) {
    log(Level::Success, message);
}

pub fn info(message: &str) {
    log(Level::Info, message);
}

pub fn debug(message: &str) {
    log(Level::Debug, message);
}

pub fn trace(message: &str) {
    log(Level::Trace, message);
}

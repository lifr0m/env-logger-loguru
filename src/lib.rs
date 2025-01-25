mod level;

use chrono::prelude::*;
use colored::Colorize;
pub use level::Level;

pub fn log_internal(level: Level, args: std::fmt::Arguments) {
    eprintln!(
        "{} | {} - {}",
        Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string().green(),
        level.colorize(&level.to_string()),
        level.colorize(&std::fmt::format(args)),
    );
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)+) => {
        $crate::log_internal($level, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Critical, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Error, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Warning, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Success, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Info, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Debug, format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::Level::Trace, format_args!($($arg)+))
    };
}

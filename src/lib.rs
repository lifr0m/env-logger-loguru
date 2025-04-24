use colored::Colorize;
use std::io::{self, Write};
use time::macros::format_description;
use time::OffsetDateTime;

pub fn format(buf: &mut env_logger::fmt::Formatter, record: &log::Record) -> io::Result<()> {
    let time = OffsetDateTime::now_local()
        .expect("can't get local datetime")
        .format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .expect("can't format datetime")
        .green();

    let level = level_colorize(record.level(), record.level().as_str());

    let place = record.target().cyan();

    let text = level_colorize(record.level(), &record.args().to_string());

    writeln!(buf, "{time} | {level:5} | {place} - {text}")
}

fn level_colorize(level: log::Level, text: &str) -> colored::ColoredString {
    match level {
        log::Level::Error => text.red().bold(),
        log::Level::Warn => text.yellow().bold(),
        log::Level::Info => text.bold(),
        log::Level::Debug => text.blue().bold(),
        log::Level::Trace => text.cyan().bold(),
    }
}

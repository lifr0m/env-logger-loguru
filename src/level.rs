use colored::{ColoredString, Colorize};

pub enum Level {
    Critical,
    Error,
    Warning,
    Success,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub(crate) fn colorize(&self, text: &str) -> ColoredString {
        match self {
            Self::Critical => text.on_red().bold(),
            Self::Error => text.red().bold(),
            Self::Warning => text.yellow().bold(),
            Self::Success => text.green().bold(),
            Self::Info => text.bold(),
            Self::Debug => text.blue().bold(),
            Self::Trace => text.cyan().bold(),
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Success => "SUCCESS",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        })
    }
}

#[cfg(feature = "std")]
use colored::Colorize;

pub fn log(level: u32, target: &str, content: &str) {
    let (log_level, log_emoji) = match level {
        1 => (log::Level::Error.to_string().red(), "❌️"),
        2 => (log::Level::Warn.to_string().yellow(), "⚠️"),
        3 => (log::Level::Info.to_string().cyan(), "❤️"),
        4 => (log::Level::Debug.to_string().purple(), "📋"),
        5 => (log::Level::Trace.to_string().normal(), "🏷"),
        _ => (log::Level::Warn.to_string().yellow(), "⚠️ unknown log_level"),
    };

    let message = format!(
        "{} {:<5} [{}] {} {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
        log_level,
        target,
        log_emoji,
        content
    );
    ink_env::debug_println!("{}", &message)
}

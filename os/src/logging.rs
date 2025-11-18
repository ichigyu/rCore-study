//! Global logger

use log::{Level, LevelFilter, Log, Metadata, Record};

/// a simple logger
struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // 决定是否应该处理给定的日志消息
        true // 所有日志都处理
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            // 为不同等级日志分配颜色
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {
        // 所有缓冲的日志都被写入目标
    }
}

/// initiate logger
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    // 静态的全局日志记录器，生命周期是整个程序生命周期
    log::set_logger(&LOGGER).unwrap();
    // 将静态的 LOGGER 实例设置为 log 库的全局日志记录器
    log::set_max_level(match option_env!("LOG") {
        // 根据环境变量LOG设置日志级别，只有等于或高于这个级别的日志才会被传递给SimpleLogger
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

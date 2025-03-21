use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_logger() -> WorkerGuard {
    let file_appender = RollingFileAppender::new(Rotation::HOURLY, "logs", "application-log.log");

    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::from_default_env()
        .add_directive(Level::INFO.into())
        .add_directive(Level::WARN.into())
        .add_directive(Level::ERROR.into())
        .add_directive(Level::DEBUG.into());

    let console_layer = fmt::Layer::default()
        .with_ansi(true)
        .with_filter(env_filter);

    let file_layer = fmt::Layer::default()
        .with_writer(non_blocking_file)
        .with_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive(Level::WARN.into())
                .add_directive(Level::ERROR.into())
                .add_directive(Level::DEBUG.into()),
        );

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}

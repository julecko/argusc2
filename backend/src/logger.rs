use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};

pub struct LoggerGuard {
    _guard: Option<WorkerGuard>,
}

pub fn init() -> LoggerGuard {
    #[cfg(debug_assertions)]
    {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"));

        fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_line_number(true)
            .with_thread_ids(false)
            .pretty()
            .init();

        tracing::debug!("Logger initialized in DEBUG mode (stdout)");
        LoggerGuard { _guard: None }
    }

    #[cfg(not(debug_assertions))]
    {
        use tracing_appender::rolling;

        std::fs::create_dir_all("logs").expect("Failed to create logs directory");

        let file_appender = rolling::daily("logs", "app.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_line_number(true)
            .with_ansi(false)
            .with_writer(non_blocking)
            .init();

        tracing::info!("Logger initialized in RELEASE mode (file: logs/app.log)");
        LoggerGuard { _guard: Some(guard) }
    }
}
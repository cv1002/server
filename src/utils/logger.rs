/// Use of tracing
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt};

use super::durations::{three_days_duration, one_hour_duration};

// --- Logger Constants --- //
pub const CLEANER: &str = "Cleaner";

/// Use this function to initialize tracing.
pub fn tracing_config_initalize() {
    // register cleaner
    tokio::spawn(async move {
        loop {
            cleanup("log");
            tokio::time::sleep(one_hour_duration()).await;
        }
    });
    // register all of subscribers
    tracing_subscriber::registry()
        // register daily_logger
        .with({
            fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(false)
                .with_writer(
                    rolling::daily("log", "tracelog_daily.log")
                        .with_max_level(tracing::Level::INFO),
                )
        })
        // initialize the tracing subscriber
        .init();
}

pub fn cleanup(path: &str) -> Option<()> {
    let dir = std::fs::read_dir(path).ok()?;
    for entry in dir {
        let Ok(entry) = entry else {
            continue;
        };
        let remove_expired_entry = || {
            let entry_expired = entry
                .metadata()
                .ok()?
                .modified()
                .ok()?
                .elapsed()
                .ok()?
                .gt(&three_days_duration());
            let entry_path = entry.path().as_os_str().to_owned();
            if entry_expired {
                match std::fs::remove_file(entry.path()) {
                    Err(error) => {
                        tracing::error!(
                            target: CLEANER,
                            "Error occured while removing entry: {:?}, error is: {}",
                            entry_path,
                            error
                        );
                    }
                    _ => {
                        tracing::info!(
                            target: CLEANER,
                            "Remove entry: {:?} success",
                            entry_path,
                        );
                    }
                };
            } else {
                tracing::info!(
                    target: CLEANER,
                    "Checked entry: {:?} not expired",
                    entry_path,
                );
            }
            Some(())
        };
        remove_expired_entry();
    }
    Some(())
}

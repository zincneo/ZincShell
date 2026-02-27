#![allow(unused)]
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};
use std::{fs, path::PathBuf};
use tracing_subscriber::{EnvFilter, fmt};

const KEEP_DAYS: i64 = 3;

fn log_dir() -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .context("Path not found")?
        .join("ZincShell"))
}

fn cleanup_old_logs(dir: &PathBuf) -> Result<()> {
    let cutoff = Local::now() - Duration::days(KEEP_DAYS);

    let entries = fs::read_dir(dir)?;

    for entry in entries.flatten() {
        let path = entry.path();
        let metadata = entry.metadata()?;
        let modified: DateTime<Local> = metadata.modified()?.into();
        if modified < cutoff {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

pub fn init() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("off").add_directive("zinc_shell=info".parse().unwrap())
    });

    #[cfg(debug_assertions)]
    {
        fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(true)
            .pretty()
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        let dir = log_dir()?;
        fs::create_dir_all(&dir)?;

        cleanup_old_logs(&dir)?;

        let file_appender = tracing_appender::rolling::daily(&dir, "zinc-shell.log");

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        fmt()
            .with_env_filter(filter)
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .init();

        Box::leak(Box::new(_guard));
    }

    Ok(())
}

mod app;
mod appfilter;
mod buffer;
mod config;
mod correction;
mod filter;
mod input;
mod tray;

use clap::Parser;
use config::AppConfig;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "keycen", about = "OS-level profanity autocorrect sensor")]
struct Cli {
    /// Run in daemon mode (no system tray, headless)
    #[arg(long)]
    daemon: bool,

    /// Path to config file (default: platform config dir)
    #[arg(long, short)]
    config: Option<String>,

    /// Enable verbose logging
    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    std::panic::set_hook(Box::new(|info| {
        log::error!("Panic: {info}");
    }));

    // Determine config path
    let config_path = cli
        .config
        .map(PathBuf::from)
        .unwrap_or_else(AppConfig::default_path);

    log::info!("Loading config from {config_path:?}");

    // Load config
    let config = match AppConfig::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to load config: {e}");
            log::info!("Using default config");
            AppConfig::default()
        }
    };

    log::info!(
        "Keycen v{} starting (daemon={}, enabled={}, mode={})",
        env!("CARGO_PKG_VERSION"),
        cli.daemon,
        config.general.enabled,
        config.general.mode
    );

    if !config.general.enabled {
        log::info!("Keycen is disabled in config. Exiting.");
        return;
    }

    let application = app::App::new(config, config_path, cli.daemon);
    application.run();
}

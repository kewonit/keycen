use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

pub struct ConfigWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<Result<Event, notify::Error>>,
    path: PathBuf,
}

impl ConfigWatcher {
    pub fn new(config_path: PathBuf) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        // Watch the parent directory (watching a single file can be unreliable)
        if let Some(parent) = config_path.parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;
        }

        Ok(ConfigWatcher {
            _watcher: watcher,
            rx,
            path: config_path,
        })
    }

    /// Check for config file changes (non-blocking)
    pub fn check_for_changes(&self) -> bool {
        while let Ok(Ok(event)) = self.rx.try_recv() {
            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {
                    if event.paths.iter().any(|p| p == &self.path) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

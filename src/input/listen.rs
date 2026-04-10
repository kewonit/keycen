use crate::appfilter::AppFilter;
use crate::buffer::{BufferAction, WordBuffer};
use crate::correction::{listen_corrector, IS_SIMULATING};
use crate::filter::{FilterResult, ProfanityFilter};
use rdev::{listen, Event, EventType, ListenError};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

/// Start the listen mode input listener
/// This function blocks the calling thread
pub fn start(
    filter: Arc<Mutex<Box<dyn ProfanityFilter>>>,
    app_filter: Arc<Mutex<AppFilter>>,
) -> Result<(), ListenError> {
    let buffer = Arc::new(Mutex::new(WordBuffer::new()));

    let callback = move |event: Event| {
        // Skip our own simulated events
        if IS_SIMULATING.load(Ordering::SeqCst) {
            return;
        }

        // Check if current app is excluded
        if let Ok(af) = app_filter.lock() {
            if af.is_excluded() {
                return;
            }
        }

        match event.event_type {
            EventType::KeyPress(key) => {
                let mut buf = buffer.lock().unwrap();
                let action = buf.process_key_press(key, event.name.as_deref());

                if let BufferAction::WordComplete(word, boundary) = action {
                    if let Ok(f) = filter.lock() {
                        match f.check(&word) {
                            FilterResult::Profane(replacement) => {
                                drop(f);
                                drop(buf);

                                // In listen mode, the word + boundary are already in the app
                                // We need to erase and retype
                                listen_corrector::correct(&word, &replacement, boundary);
                            }
                            FilterResult::Clean => {}
                        }
                    }
                }
            }
            EventType::KeyRelease(key) => {
                let mut buf = buffer.lock().unwrap();
                buf.process_key_release(key);
            }
            // Mouse events could invalidate the buffer (user clicked somewhere)
            EventType::ButtonPress(_) => {
                let mut buf = buffer.lock().unwrap();
                buf.clear();
            }
            _ => {}
        }
    };

    log::info!("Starting listen mode input listener");
    listen(callback)
}

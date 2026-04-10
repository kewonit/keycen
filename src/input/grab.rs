use crate::appfilter::AppFilter;
use crate::buffer::{BufferAction, WordBuffer};
use crate::correction::{grab_corrector, CORRECTION_IN_PROGRESS, IS_SIMULATING};
use crate::filter::{FilterResult, ProfanityFilter};
use rdev::{grab, Event, EventType, GrabError};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Cooldown after a correction to prevent re-triggering on replacement words
const CORRECTION_COOLDOWN_MS: u128 = 500;

/// Start the grab mode input listener
/// This function blocks the calling thread
pub fn start(
    filter: Arc<Mutex<Box<dyn ProfanityFilter>>>,
    app_filter: Arc<Mutex<AppFilter>>,
) -> Result<(), GrabError> {
    let buffer = Arc::new(Mutex::new(WordBuffer::new()));
    let last_correction = Arc::new(Mutex::new(
        Instant::now()
            .checked_sub(std::time::Duration::from_secs(10))
            .unwrap_or(Instant::now()),
    ));

    let callback = move |event: Event| -> Option<Event> {
        // Skip our own simulated events
        if IS_SIMULATING.load(Ordering::SeqCst) {
            return Some(event);
        }

        // While a correction is in progress, pass keys through but don't buffer them.
        // This prevents user keystrokes from interleaving with correction output.
        if CORRECTION_IN_PROGRESS.load(Ordering::SeqCst) {
            return Some(event);
        }

        // Check if current app is excluded
        if let Ok(af) = app_filter.lock() {
            if af.is_excluded() {
                return Some(event);
            }
        }

        match event.event_type {
            EventType::KeyPress(key) => {
                let mut buf = buffer.lock().unwrap();
                let action = buf.process_key_press(key, event.name.as_deref());

                match action {
                    BufferAction::WordComplete(word, boundary) => {
                        // Skip if we just corrected (prevents re-flagging replacement words)
                        if let Ok(lc) = last_correction.lock() {
                            if lc.elapsed().as_millis() < CORRECTION_COOLDOWN_MS {
                                return Some(event);
                            }
                        }

                        // Check profanity
                        if let Ok(f) = filter.lock() {
                            match f.check(&word) {
                                FilterResult::Profane(replacement) => {
                                    drop(f);
                                    // Reset buffer immediately after detecting profanity
                                    buf.clear();
                                    drop(buf);

                                    // Record correction time
                                    if let Ok(mut lc) = last_correction.lock() {
                                        *lc = Instant::now();
                                    }

                                    // Spawn correction in a separate thread to avoid
                                    // blocking the keyboard hook callback (Windows removes
                                    // hooks that take too long)
                                    let word_clone = word.clone();
                                    let replacement_clone = replacement.clone();
                                    std::thread::spawn(move || {
                                        grab_corrector::correct(
                                            &word_clone,
                                            &replacement_clone,
                                            boundary,
                                        );
                                    });

                                    // Pass the boundary character through
                                    return Some(event);
                                }
                                FilterResult::Clean => {}
                            }
                        }
                        Some(event)
                    }
                    _ => Some(event),
                }
            }
            EventType::KeyRelease(key) => {
                let mut buf = buffer.lock().unwrap();
                buf.process_key_release(key);
                Some(event)
            }
            // Non-keyboard events pass through
            _ => Some(event),
        }
    };

    log::info!("Starting grab mode input listener");
    grab(callback)
}

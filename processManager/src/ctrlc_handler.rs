use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;

pub static RUNNING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(true));

pub fn setup_ctrlc_handler() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
    })?;
    Ok(())
}

pub fn exiting_loop() {
    RUNNING.store(false, Ordering::SeqCst);
}
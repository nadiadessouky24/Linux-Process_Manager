pub use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
pub use ctrlc;
pub use once_cell::sync::Lazy;
pub use std::{thread, time::Duration};

pub static RUNNING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(true)));

pub fn exiting_loop() {
    let running = Arc::clone(&*RUNNING);
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting CTRL-C handler");
}
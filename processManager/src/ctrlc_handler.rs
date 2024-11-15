use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use ctrlc;
use once_cell::sync::Lazy;
use std::{thread, time::Duration};

pub static RUNNING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(true)));

pub fn exiting_loop() {
    let running = Arc::clone(&*RUNNING);
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);

        thread::sleep(Duration::from_secs(15));

        running.store(true, Ordering::SeqCst);
    }).expect("Error setting CTRL-C handler");
}

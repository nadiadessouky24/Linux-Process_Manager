use systemstat::{System, Platform};
use std::sync::atomic::AtomicBool;
use std::{thread, time::Duration};
use std::io::{self, Write}; // Import Write trait
use crate::common::{Arc,ctrlc,Ordering};
use crate::ctrlc_handler::{exiting_loop, RUNNING}; 

pub fn display_load_avg()-> Result<(), Box<dyn std::error::Error>> {
    let sys = System::new();
    let update_interval = Duration::from_secs(5);

    while  RUNNING.load(Ordering::SeqCst) {
        match sys.load_average() {
            Ok(loadavg) => {
                print!("\rload average: {:.2}, {:.2}, {:.2}", loadavg.one, loadavg.five, loadavg.fifteen);
                std::io::stdout().flush().unwrap(); // Ensure the output is written immediately
            }
            Err(e) => eprintln!("\rFailed to get load average: {}", e),
        }

        thread::sleep(update_interval);
    }
    Ok(())

}
use std::{process, thread, time::Duration};
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult};
use sysinfo::{ProcessExt, ProcessStatus, System, SystemExt};
use clap::{Command, Arg, ArgAction};  

/// Function to count zombie processes using sysinfo
fn count_zombie_processes() -> usize {
    let mut system = System::new_all();
    system.refresh_all(); // Update system info

    let zombie_count = system.processes()
        .values()
        .filter(|process| process.status() == ProcessStatus::Zombie)
        .count();

    zombie_count
}

fn main() {
    // Define the command-line arguments using the updated clap API
    let matches = Command::new("Zombie Process Counter")  // Updated to `Command`
        .version("1.0")
        .author("amalfouda")
        .long_about("Counts the number of zombie processes on the system.") // Updated `long_about`
        .arg(
            Arg::new("zombie")
                .short('z')
                .long("zombie")
                .help("Counts zombie processes")
                .action(ArgAction::SetTrue),  // Mark as flag with no value
        )
        .get_matches();  // <-- Make sure this is correctly placed

    
    if matches.get_one::<bool>("zombie").copied().unwrap_or(false) {
        println!("Counting zombie processes...");

        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                // Child process immediately exits, creating a zombie
                println!("Child process (PID: {}) exiting to become a zombie...", process::id());
                process::exit(0);
            }
            Ok(ForkResult::Parent { .. }) => {
                // Parent process sleeps, leaving the child as a zombie
                println!("Parent process (PID: {}) sleeping...", process::id());
                thread::sleep(Duration::from_secs(5)); // Sleep to allow time for testing

                // Count zombie processes after creating a zombie
                let zombie_count = count_zombie_processes();
                println!("Number of zombie processes: {}", zombie_count);

                // Wait to clean up the zombie process
                let _ = wait(); // This reaps the child process, removing it from zombie state
            }
            Err(e) => eprintln!("Fork failed: {}", e),
        }
    } else {
        println!("Use the -z or --zombie flag to count zombie processes.");
    }
}

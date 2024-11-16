use sysinfo::{ProcessExt, System, SystemExt};
use std::io::{self, Write}; 

pub fn display_zombie_processes() -> usize {
    let mut system = System::new_all();
    system.refresh_all();

    let mut zombie_count = 0;

    for (_, process) in system.processes() {
        if let sysinfo::ProcessStatus::Zombie = process.status() {
            println!("Zombie Process: PID {}", process.pid());
            zombie_count += 1;
        }
    }

    zombie_count
}



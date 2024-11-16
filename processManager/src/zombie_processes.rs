use sysinfo::{ProcessExt, System, SystemExt};
use std::io::{self, Write}; 

fn display_zombie_processes() {
    let mut system = System::new_all();
    system.refresh_all();

    let mut zombie_found = false;
    
    
    for (_, process) in system.processes() {
        if let sysinfo::ProcessStatus::Zombie = process.status() {
            println!("Zombie Process: PID {}", process.pid());
            zombie_found = true;
        }
    }

    
    if !zombie_found {
        println!("No zombie processes found.");
    }
}




use std::io::stdin;
use crate::ctrlc_handler::RUNNING;
use crate::display_filtered::display_filtered_processes;
use crate::filtering::filter_processes;
use sysinfo::{System, SystemExt};
use std::sync::atomic::Ordering;

pub fn handle_filter_process() -> Result<(), Box<dyn std::error::Error>> {
    let mut sysinfo = System::new_all();
    sysinfo.refresh_all();

    println!("Filter Options:");
    println!("1. Filter by PID");
    println!("2. Filter by Process Name (procname)");
    println!("3. Filter by CPU Usage");
    println!("4. Filter by All Criteria");
    println!("Enter your choice (1-4):");

    let mut choice = String::new();
    stdin().read_line(&mut choice)?;

    let choice = choice.trim().parse::<u8>().unwrap_or(0);

    let mut filtered_pid = None;
    let mut filtered_procname: Option<String> = None;
    let mut min_cpu_usage = None;

    match choice {
        1 => {
            println!("Enter PID to filter:");
            let mut pid_input = String::new();
            stdin().read_line(&mut pid_input)?;
            filtered_pid = pid_input.trim().parse::<i32>().ok();
        }
        2 => {
            println!("Enter process name to filter by:");
            let mut procname_input = String::new();
            stdin().read_line(&mut procname_input)?;
            filtered_procname = Some(procname_input.trim().to_string());
        }
        3 => {
            println!("Enter minimum CPU usage to filter by:");
            let mut cpu_input = String::new();
            stdin().read_line(&mut cpu_input)?;
            min_cpu_usage = cpu_input.trim().parse::<f32>().ok();
        }
        4 => {
            println!("Enter PID to filter (or leave blank):");
            let mut pid_input = String::new();
            stdin().read_line(&mut pid_input)?;
            filtered_pid = pid_input.trim().parse::<i32>().ok();

            println!("Enter process name to filter by (or leave blank):");
            let mut procname_input = String::new();
            stdin().read_line(&mut procname_input)?;
            if !procname_input.trim().is_empty() {
                filtered_procname = Some(procname_input.trim().to_string());
            }

            println!("Enter minimum CPU usage to filter by (or leave blank):");
            let mut cpu_input = String::new();
            stdin().read_line(&mut cpu_input)?;
            min_cpu_usage = cpu_input.trim().parse::<f32>().ok();
        }
        _ => {
            println!("Invalid choice. Returning to main menu.");
            return Ok(());
        }
    }

    // Filter processes based on user criteria
    let filtered_procname_ref = filtered_procname.as_deref(); // Convert to &str if Some
    let filtered_processes = filter_processes(&sysinfo, filtered_pid, filtered_procname_ref, min_cpu_usage);

    // Display filtered processes
    RUNNING.store(true, Ordering::SeqCst);
    display_filtered_processes(filtered_pid, filtered_procname_ref, min_cpu_usage)?;

    Ok(())
}

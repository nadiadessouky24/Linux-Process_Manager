use std::{io::{stdout, Write}, sync::{atomic::AtomicBool, Arc}};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
use std::{thread, time::Duration};
use systemstat::{System as StatSystem, Platform};
use sysinfo::{System, SystemExt, ProcessExt};
use crate::{common::ctrlc, ctrlc_handler::{exiting_loop, RUNNING}};
use crate::common::Ordering;

pub fn display_process_info() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system utilities
    let mut sysinfo = System::new_all();
    let stat_sys = StatSystem::new();
    let update_interval = Duration::from_secs(5);
    let mut stdout = stdout();
    sysinfo.refresh_all();
    thread::sleep(Duration::from_secs(2));

    while RUNNING.load(Ordering::SeqCst) {
        // Clear the terminal and move the cursor to the top-left
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Fetch load averages
        let loadavg = stat_sys.load_average().ok();
        let (one, five, fifteen) = if let Some(load) = loadavg {
            (load.one, load.five, load.fifteen)
        } else {
            (0.0, 0.0, 0.0)
        };

        // Count processes by state
        let mut total_tasks = 0;
        let mut running = 0;
        let mut sleeping = 0;
        let mut stopped = 0;
        let mut zombie = 0;

        // Iterate over processes using `sysinfo`
        for process in sysinfo.processes() {
            let proc = process.1;
            total_tasks += 1;
            match proc.status() {
                sysinfo::ProcessStatus::Run => running += 1,
                sysinfo::ProcessStatus::Sleep => sleeping += 1,
                sysinfo::ProcessStatus::Stop => stopped += 1,
                sysinfo::ProcessStatus::Zombie => zombie += 1,
                _ => {}
            }
        }

        // Display load averages and task summary
        println!(
            "Load Average: {:.2}, {:.2}, {:.2}\n\
            Tasks: Total: {}, Running: {}, Sleeping: {}, Stopped: {}, Zombie: {}\n",
            one, five, fifteen, total_tasks, running, sleeping, stopped, zombie
        );

        // Refresh sysinfo data
        sysinfo.refresh_all();

        // Print process information
        println!(
            "{:<10} {:<30} {:<10} {:<10} {:<10}",
            "PID", "Name", "Status", "CPU (%)", "Mem (KB)"
        );

        for process in sysinfo.processes() {
            let pid = process.0;
            let proc = process.1;

            println!(
                "{:<10} {:<30} {:<10} {:<10.2} {:<10}",
                pid,
                truncate_string(proc.name(), 30),
                format!("{:?}", proc.status()),
                proc.cpu_usage(),
                proc.memory() / 1024
            );
        }

        stdout.flush()?; // Ensure the output is flushed
        thread::sleep(update_interval);
    }
    Ok(())
}

// Helper function to truncate long strings for the "Name" column
fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() > max_length {
        format!("{}...", &input[..max_length - 3])
    } else {
        input.to_string()
    }
}
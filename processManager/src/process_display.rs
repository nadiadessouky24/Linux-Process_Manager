use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
use std::{thread, time::Duration};
use systemstat::{System as StatSystem, Platform};
use procfs::process::all_processes;
use sysinfo::{System, Process};

pub fn display_process_info() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system utilities
    let mut sysinfo = System::new_all();
    let stat_sys = StatSystem::new();
    let update_interval = Duration::from_secs(5);
    let mut stdout = stdout();

    loop {
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

        if let Ok(processes) = all_processes() {
            for process_result in processes {
                if let Ok(process) = process_result {
                    if let Ok(stat) = process.stat() {
                        total_tasks += 1;
                        match stat.state {
                            'R' => running += 1,
                            'S' => sleeping += 1,
                            'T' => stopped += 1,
                            'Z' => zombie += 1,
                            _ => {}
                        }
                    }
                }
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

        for (pid, process) in sysinfo.processes() {
            println!(
                "{:<10} {:<30} {:<10} {:<15.2} {:<15}",
                pid,
                format!("{}", process.name().to_string_lossy()),
                format!("{:?}", process.status()),
                process.cpu_usage(),
                process.memory() / 1024
            );
        }

        stdout.flush()?; // Ensure the output is flushed
        thread::sleep(update_interval);
    }
}

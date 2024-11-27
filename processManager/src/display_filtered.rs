use std::{io::{stdout, Write}, sync::{atomic::AtomicBool, Arc}};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
use std::{thread, time::Duration};
use systemstat::{System as StatSystem, Platform};
use sysinfo::{System, SystemExt, ProcessExt};
use crate::{common::ctrlc, ctrlc_handler::{exiting_loop, RUNNING}};
use crate::common::Ordering;
use std::io::stdin;



pub fn display_filtered_processes(
    filtered_pid: Option<i32>,
    filtered_procname: Option<&str>,
    min_cpu_usage: Option<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sysinfo = System::new_all();
    let mut stdout = stdout();
    let update_interval = Duration::from_secs(5);

    while RUNNING.load(Ordering::SeqCst) {
        // Refresh system information
        sysinfo.refresh_all();

        // Clear the terminal and move cursor to the top-left
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Fetch load averages
        let stat_sys = StatSystem::new();
        let loadavg = stat_sys.load_average().ok();
        let (one, five, fifteen) = if let Some(load) = loadavg {
            (load.one, load.five, load.fifteen)
        } else {
            (0.0, 0.0, 0.0)
        };

        // Filter processes and categorize them
        let filtered_processes: Vec<_> = sysinfo
            .processes()
            .iter()
            .filter(|(&pid, proc)| {
                filtered_pid.map_or(false, |p| p == pid)
                    || filtered_procname.map_or(false, |name| proc.name().contains(name))
                    || min_cpu_usage.map_or(false, |cpu| proc.cpu_usage() >= cpu)
            })
            .collect();

        let mut total_tasks = filtered_processes.len();
        let mut running = 0;
        let mut sleeping = 0;
        let mut stopped = 0;
        let mut zombie = 0;

        for (_, proc) in &filtered_processes {
            match proc.status() {
                sysinfo::ProcessStatus::Run => running += 1,
                sysinfo::ProcessStatus::Sleep => sleeping += 1,
                sysinfo::ProcessStatus::Stop => stopped += 1,
                sysinfo::ProcessStatus::Zombie => zombie += 1,
                _ => {}
            }
        }

        // Display task summary and load averages
        println!(
            "Load Average: {:.2}, {:.2}, {:.2}\n\
            Tasks (Filtered): Total: {}, Running: {}, Sleeping: {}, Stopped: {}, Zombie: {}\n",
            one, five, fifteen, total_tasks, running, sleeping, stopped, zombie
        );

        // Print process header
        println!(
            "{:<10} {:<30} {:<10} {:<10} {:<10}",
            "PID", "Name", "Status", "CPU (%)", "Mem (KB)"
        );

        // Display filtered process details
        for (&pid, proc) in &filtered_processes {
            println!(
                "{:<10} {:<30} {:<10} {:<10.2} {:<10}",
                pid,
                truncate_string(proc.name(), 30),
                format!("{:?}", proc.status()),
                proc.cpu_usage(),
                proc.memory() / 1024
            );
        }

        stdout.flush()?; // Flush output
        thread::sleep(update_interval);
    }

    Ok(())
}


fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() > max_length {
        format!("{}...", &input[..max_length - 3])
    } else {
        input.to_string()
    }
}
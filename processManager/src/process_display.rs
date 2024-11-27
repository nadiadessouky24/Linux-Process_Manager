
use std::{io::{stdout, Write}, sync::{atomic::AtomicBool, Arc}, thread, time::Duration};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
//use sysinfo::System;
use sysinfo::{System, SystemExt, ProcessExt};
use systemstat::{System as StatSystem, Platform};
use colored::Colorize;
use prettytable::{Table, row, format};
use libc;
use crate::{common::ctrlc, ctrlc_handler::{exiting_loop, RUNNING}};
use crate::common::Ordering;
use std::fs;


fn get_cpu_core_for_pid(pid: i32) -> Option<i32> {
    let stat_path = format!("/proc/{}/stat", pid);
    if let Ok(stat_content) = fs::read_to_string(stat_path) {
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() >= 39 {
            return fields[38].parse().ok(); // 39th field is the core ID
        }
    }
    None
}



fn update_table(system: &System, stat_sys: &StatSystem) -> Table {
    // Create a new table with custom formatting
    let mut table = Table::new();
    table.set_format(
        format::FormatBuilder::new()
            .column_separator('|') // Gridlines between columns
            .borders('─') // Solid-line borders for the table
            .separator(
                format::LinePosition::Top,
                format::LineSeparator::new('─', '┌', '┬', '┐'),
            )
            .separator(
                format::LinePosition::Bottom,
                format::LineSeparator::new('─', '└', '┴', '┘'),
            )
            .separator(
                format::LinePosition::Intern,
                format::LineSeparator::new('─', '├', '┼', '┤'),
            )
            .padding(1, 1) // Padding on left and right of cells
            .build(),
    );

    // Add headers for load averages and task summary
    table.add_row(row![
        format!("{:^10}", "SYSTEM STATS").bold().underline().green()
    ]);
    table.add_row(row![
        format!("{:^25}", "Load Avg (1m, 5m, 15m)".green()),
        format!("{:^15}", "Total Tasks".green()),
        format!("{:^10}", "Running".green()),
        format!("{:^10}", "Sleeping".green()),
        format!("{:^10}", "Stopped".green()),
        format!("{:^10}", "Zombie".green())
    ]);

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

    for process in system.processes() {
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

    // Add system stats row
    table.add_row(row![
        format!("{:^25}", format!("{:.2}, {:.2}, {:.2}", one, five, fifteen)),
        format!("{:^15}", total_tasks.to_string()),
        format!("{:^10}", running.to_string()),
        format!("{:^10}", sleeping.to_string()),
        format!("{:^10}", stopped.to_string()),
        format!("{:^10}", zombie.to_string())
    ]);

    // Add process details section header
    table.add_row(row![
        format!("{:^10}", "PROCESS DETAILS").bold().underline().green()
    ]);
    table.add_row(row![
        format!("{:^10}", "PID".green()),
        format!("{:^20}", "NAME".green()),
        format!("{:^10}", "STATUS".green()),
        format!("{:^10}", "CPU %".green()),
        format!("{:^10}", "MEMORY".green()),
        format!("{:^10}", "CORE".green())
    ]);

    // Add rows with process data
    for (pid, process) in system.processes() {
        table.add_row(row![
            format!("{:^10}", pid.to_string()),
            format!("{:^20}", process.name()),
            format!("{:^10}", format!("{:?}", process.status())),
            format!("{:^10}", format!("{:.2}", process.cpu_usage())),
            format!("{:^10}", (process.memory()/1024).to_string()),
            format!(
                "{:^10}",
                get_cpu_core_for_pid(process.pid())
                    .map(|core| core.to_string()) // Convert `Some(core)` to a `String`
                    .unwrap_or_else(|| "N/A".to_string()) // Handle `None` gracefully
            )
        ]);
    }

    table
}


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
     
        // Refresh sysinfo data
        sysinfo.refresh_all();

        // Generate and display the updated table
        let table = update_table(&sysinfo, &stat_sys);
        table.printstd();

        stdout.flush()?; // Ensure the output is flushed
        thread::sleep(update_interval);
    }
    Ok(())
}

use fltk::{
    app,
    prelude::*,
    *,
    table::*,
    draw,
    enums::{Align, Color, FrameType, Font},
    frame::Frame,
    input::Input,
    button::Button,
};
use systemstat::{System as StatSystem, Platform};
use sysinfo::{System, SystemExt, ProcessExt};
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
use crate::ctrlc_handler::RUNNING;
use std::sync::atomic::Ordering;
use std::fs;
use libc;

pub struct ProcessDisplay {
    sysinfo: System,
    stat_sys: StatSystem,
    processes: Vec<ProcessInfo>,
    load_avg: (f32, f32, f32),
    task_summary: TaskSummary,
    filters: ProcessFilters,
}

#[derive(Clone)]
struct ProcessInfo {
    pid: i32,
    name: String,
    status: String,
    cpu_usage: f32,
    memory: u64,
    core: Option<i32>,
}

struct TaskSummary {
    total: usize,
    running: usize,
    sleeping: usize,
    stopped: usize,
    zombie: usize,
}

struct ProcessFilters {
    pid: Option<String>,
    name: Option<String>,
    status: Option<String>,
    cpu_min: Option<f32>,
    memory_min: Option<u64>,
}

impl ProcessDisplay {
    pub fn new() -> Self {
        let mut sysinfo = System::new_all();
        let stat_sys = StatSystem::new();
        sysinfo.refresh_all();
        
        Self {
            sysinfo,
            stat_sys,
            processes: Vec::new(),
            load_avg: (0.0, 0.0, 0.0),
            task_summary: TaskSummary {
                total: 0,
                running: 0,
                sleeping: 0,
                stopped: 0,
                zombie: 0,
            },
            filters: ProcessFilters {
                pid: None,
                name: None,
                status: None,
                cpu_min: None,
                memory_min: None,
            },
        }
    }

    fn get_cpu_core_for_pid(pid: i32) -> Option<i32> {
        let stat_path = format!("/proc/{}/stat", pid);
        if let Ok(stat_content) = fs::read_to_string(stat_path) {
            let fields: Vec<&str> = stat_content.split_whitespace().collect();
            if fields.len() >= 39 {
                return fields[38].parse().ok();
            }
        }
        None
    }

    pub fn update(&mut self) {
        self.sysinfo.refresh_all();
        
        // Update load averages
        if let Ok(load) = self.stat_sys.load_average() {
            self.load_avg = (load.one, load.five, load.fifteen);
        }

        // Update task summary and process list
        self.processes.clear();
        self.task_summary = TaskSummary {
            total: 0,
            running: 0,
            sleeping: 0,
            stopped: 0,
            zombie: 0,
        };

        for (pid, proc) in self.sysinfo.processes() {
            self.task_summary.total += 1;
            match proc.status() {
                sysinfo::ProcessStatus::Run => self.task_summary.running += 1,
                sysinfo::ProcessStatus::Sleep => self.task_summary.sleeping += 1,
                sysinfo::ProcessStatus::Stop => self.task_summary.stopped += 1,
                sysinfo::ProcessStatus::Zombie => self.task_summary.zombie += 1,
                _ => {}
            }

            self.processes.push(ProcessInfo {
                pid: *pid,
                name: proc.name().to_string(),
                status: format!("{:?}", proc.status()),
                cpu_usage: proc.cpu_usage(),
                memory: proc.memory() / 1024,
                core: Self::get_cpu_core_for_pid(*pid),
            });
        }
    }

    fn apply_filters(&mut self) {
        let mut filtered_processes = Vec::new();
        
        for (pid, proc) in self.sysinfo.processes() {
            let matches = match (&self.filters.pid, &self.filters.name, &self.filters.cpu_min) {
                (Some(pid_filter), _, _) if !pid.to_string().contains(pid_filter) => false,
                (_, Some(name_filter), _) if !proc.name().to_lowercase().contains(&name_filter.to_lowercase()) => false,
                (_, _, Some(cpu_min)) if proc.cpu_usage() < *cpu_min => false,
                _ => true
            };

            if matches {
                filtered_processes.push(ProcessInfo {
                    pid: *pid,
                    name: proc.name().to_string(),
                    status: format!("{:?}", proc.status()),
                    cpu_usage: proc.cpu_usage(),
                    memory: proc.memory() / 1024,
                    core: Self::get_cpu_core_for_pid(*pid),
                });
            }
        }
        
        self.processes = filtered_processes;
    }
}

pub fn display_process_info_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(800, 600)
        .with_label("Process Information");
    wind.set_color(Color::from_rgb(245, 245, 245));

    // Create header frame for system info
    let mut header = frame::Frame::default()
        .with_size(780, 60)
        .with_pos(10, 10);
    header.set_label_color(Color::from_rgb(51, 122, 183));

    // Create table for processes
    let mut table = Table::default()
        .with_size(780, 520)
        .with_pos(10, 70);
    table.set_selection_color(Color::from_rgb(200, 215, 255));
    
    // Start with 0 rows (besides header) until we have data
    table.set_rows(1);  // This is correct - header row only initially
    table.set_cols(6);
    table.set_col_header(true);
    table.set_col_width_all(130);
    table.set_row_height_all(25);
    
    let process_display = Rc::new(RefCell::new(ProcessDisplay::new()));
    let process_display_clone = process_display.clone();
    
    // Add this: Create an input dialog for filters
    let create_filter_dialog = |title: &str| {
        let mut dialog = window::Window::default()
            .with_size(300, 200)
            .with_label(title)
            .center_screen();
        dialog.make_modal(true);
        
        let mut pack = group::Pack::new(10, 10, 280, 180, "");
        pack.set_spacing(10);
        
        let mut description = frame::Frame::default()
            .with_size(280, 40);
        description.set_label_size(11);
        match title {
            "Filter by PID" => description.set_label("Enter a PID number to filter processes"),
            "Filter by Name" => description.set_label("Enter text to filter process names"),
            "Filter by Status" => description.set_label("Enter status (e.g., Sleep, Run)"),
            "Filter by CPU (%)" => description.set_label("Enter minimum CPU % to show"),
            "Filter by Memory (KB)" => description.set_label("Enter minimum Memory in KB"),
            _ => description.set_label("Enter value to filter"),
        }
        
        let mut input = input::Input::default()
            .with_size(280, 25);
        input.set_label("Filter value:");
        input.set_label_size(12);
        
        let mut ok_button = button::Button::default()
            .with_size(100, 30)
            .with_pos(100, 150)
            .with_label("Apply Filter");
        ok_button.set_color(Color::from_rgb(91, 192, 222));
        ok_button.set_label_color(Color::White);
            
        pack.end();
        dialog.end();
        (dialog, input, ok_button)
    };

    let process_display_for_click = process_display.clone();
    
    table.draw_cell(move |t, mut ctx, row, col, x, y, w, h| {
        match ctx {
            TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
            TableContext::ColHeader => {
                let headers = ["PID", "Name", "Status", "CPU (%)", "Memory (KB)", "Core"];
                if let Some(header) = headers.get(col as usize) {
                    draw_header(t, &mut ctx, x, y, w, h, header);
                }
                
                // Handle click events separately
                if app::event() == enums::Event::Push && 
                   app::event_inside(x, y, w, h) && 
                   row == 0  // Ensure we're clicking on header row
                {
                    let pd = process_display_for_click.clone();
                    let (mut dialog, mut input, mut ok_button) = create_filter_dialog(&format!("Filter by {}", headers[col as usize]));
                    
                    // Position dialog near the clicked header
                    let (mouse_x, mouse_y) = app::event_coords();
                    dialog.set_pos(mouse_x, mouse_y);
                    
                    let col = col;  // Clone for closure
                    let mut dialog_clone = dialog.clone();  // Made dialog_clone mutable
                    ok_button.set_callback(move |_| {
                        let filter_value = input.value();
                        let mut pd = pd.borrow_mut();
                        
                        if !filter_value.is_empty() {
                            match col {
                                0 => pd.filters.pid = Some(filter_value),
                                1 => pd.filters.name = Some(filter_value),
                                3 => pd.filters.cpu_min = filter_value.parse().ok(),
                                4 => pd.filters.memory_min = filter_value.parse().ok(),
                                _ => {}
                            }
                        } else {
                            match col {
                                0 => pd.filters.pid = None,
                                1 => pd.filters.name = None,
                                3 => pd.filters.cpu_min = None,
                                4 => pd.filters.memory_min = None,
                                _ => {}
                            }
                        }
                        pd.apply_filters();
                        dialog_clone.hide();  // Use the cloned dialog here
                    });
                    
                    dialog.show();  // Use the original dialog here
                }
            },
            TableContext::Cell => {
                let pd = process_display_clone.borrow();
                if let Some(process) = pd.processes.get((row as usize) - 1) {
                    let value = match col {
                        0 => process.pid.to_string(),
                        1 => process.name.clone(),
                        2 => process.status.clone(),
                        3 => format!("{:.2}", process.cpu_usage),
                        4 => process.memory.to_string(),
                        5 => process.core.map_or("N/A".to_string(), |c| c.to_string()),
                        _ => String::new(),
                    };
                    draw_cell(t, &mut ctx, x, y, w, h, &value);
                }
            },
            _ => (),
        }
    });

    // Add this to make the table interactive
    table.handle(move |_, ev| {
        match ev {
            enums::Event::Push => {
                true  // Handle the event
            },
            _ => false,
        }
    });

    wind.end();
    wind.show();

    // Create update channel
    let (sender, receiver) = app::channel::<()>();

    // Spawn update thread
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(2));
            sender.send(());
        }
    });

    while app.wait() {
        if let Some(_) = receiver.recv() {
            // Update data
            process_display.borrow_mut().update();
            
            // Get current data for UI updates
            let pd = process_display.borrow();
            
            // Debug print
            println!("Updating table with {} processes", pd.processes.len());
            
            // Update header with system info
            header.set_label(&format!(
                "Load Average: {:.2}, {:.2}, {:.2}\nTasks: Total: {}, Running: {}, Sleeping: {}, Stopped: {}, Zombie: {}",
                pd.load_avg.0, pd.load_avg.1, pd.load_avg.2,
                pd.task_summary.total,
                pd.task_summary.running,
                pd.task_summary.sleeping,
                pd.task_summary.stopped,
                pd.task_summary.zombie
            ));

            // Update table rows and trigger redraw
            let num_rows = pd.processes.len() as i32 + 1;  // +1 for header
            table.set_rows(num_rows);
            
            // Force table to redraw completely
            table.redraw();
            app::redraw();  // Force a complete redraw of the application
        }
    }

    Ok(())
}

pub fn display_process_info_cli() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system utilities
    let mut sysinfo = System::new_all();
    let stat_sys = StatSystem::new();
    let update_interval = Duration::from_secs(5);
    let mut stdout = stdout();
    sysinfo.refresh_all();
    thread::sleep(Duration::from_secs(2));

    while RUNNING.load(Ordering::SeqCst) {
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

        println!(
            "Load Average: {:.2}, {:.2}, {:.2}\n\
            Tasks: Total: {}, Running: {}, Sleeping: {}, Stopped: {}, Zombie: {}\n",
            one, five, fifteen, total_tasks, running, sleeping, stopped, zombie
        );

        sysinfo.refresh_all();

        println!(
            "{:<10} {:<30} {:<10} {:<10} {:<10} {:<10}",
            "PID", "Name", "Status", "CPU (%)", "Mem (KB)", "Core"
        );

        for process in sysinfo.processes() {
            let pid = process.0;
            let proc = process.1;
            let core = ProcessDisplay::get_cpu_core_for_pid(*pid)
                .map_or("N/A".to_string(), |c| c.to_string());

            println!(
                "{:<10} {:<30} {:<10} {:<10.2} {:<10} {:<10}",
                pid,
                truncate_string(proc.name(), 30),
                format!("{:?}", proc.status()),
                proc.cpu_usage(),
                proc.memory() / 1024,
                core
            );
        }

        stdout.flush()?;
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

// Update the draw helper functions
fn draw_header(_table: &mut Table, _ctx: &mut TableContext, x: i32, y: i32, w: i32, h: i32, text: &str) {
    draw::push_clip(x, y, w, h);
    draw::set_draw_color(Color::from_rgb(51, 122, 183));
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::from_rgb(51, 122, 183));
    draw::set_draw_color(Color::White);
    draw::draw_text2(text, x, y, w, h, Align::Center);
    draw::pop_clip();
}

fn draw_cell(_table: &mut Table, _ctx: &mut TableContext, x: i32, y: i32, w: i32, h: i32, text: &str) {
    draw::push_clip(x, y, w, h);
    
    let row_color = if (y / h) % 2 == 0 {
        Color::from_rgb(240, 240, 255)
    } else {
        Color::White
    };
    
    draw::set_draw_color(row_color);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, row_color);
    
    let is_cpu_col = (x / w) == 3;
    if is_cpu_col {
        if let Ok(cpu_value) = text.parse::<f32>() {
            draw::set_draw_color(if cpu_value > 50.0 {
                Color::from_rgb(217, 83, 79)
            } else if cpu_value > 20.0 {
                Color::from_rgb(240, 173, 78)
            } else {
                Color::from_rgb(92, 184, 92)
            });
        }
    } else {
        draw::set_draw_color(Color::Black);
    }
    
    draw::draw_text2(text, x, y, w, h, Align::Center);
    draw::pop_clip();
}

pub fn display_filter_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(400, 350)
        .with_label("Filter Processes")
        .center_screen();

    let mut pack = group::Pack::new(10, 10, 380, 330, "");
    pack.set_spacing(15);

    // Add a title/description
    let mut description = Frame::default()
        .with_size(380, 40)
        .with_label("Enter values to filter processes.\nLeave blank to ignore that filter.");
    description.set_label_size(12);
    
    // PID filter with visible label
    let mut pid_input = Input::default()
        .with_size(380, 30);
    pid_input.set_label("Process ID (PID):");
    pid_input.set_label_size(12);
    pid_input.set_label_type(fltk::enums::LabelType::Normal);
    pid_input.set_align(fltk::enums::Align::Top | fltk::enums::Align::Left);
    
    // Name filter with visible label
    let mut name_input = Input::default()
        .with_size(380, 30);
    name_input.set_label("Process Name:");
    name_input.set_label_size(12);
    name_input.set_label_type(fltk::enums::LabelType::Normal);
    name_input.set_align(fltk::enums::Align::Top | fltk::enums::Align::Left);
    
    // CPU filter with visible label
    let mut cpu_input = Input::default()
        .with_size(380, 30);
    cpu_input.set_label("Minimum CPU Usage (%):");
    cpu_input.set_label_size(12);
    cpu_input.set_label_type(fltk::enums::LabelType::Normal);
    cpu_input.set_align(fltk::enums::Align::Top | fltk::enums::Align::Left);
    
    // Memory filter with visible label
    let mut mem_input = Input::default()
        .with_size(380, 30);
    mem_input.set_label("Minimum Memory (KB):");
    mem_input.set_label_size(12);
    mem_input.set_label_type(fltk::enums::LabelType::Normal);
    mem_input.set_align(fltk::enums::Align::Top | fltk::enums::Align::Left);

    // Apply button
    let mut apply_btn = Button::default()
        .with_size(380, 40)
        .with_label("Apply Filters");
    apply_btn.set_color(Color::from_rgb(91, 192, 222));
    apply_btn.set_label_color(Color::White);

    pack.end();
    wind.end();
    wind.show();

    let process_display = Rc::new(RefCell::new(ProcessDisplay::new()));
    
    apply_btn.set_callback(move |_| {
        let mut pd = process_display.borrow_mut();
        
        pd.filters.pid = if pid_input.value().is_empty() { None } else { Some(pid_input.value()) };
        pd.filters.name = if name_input.value().is_empty() { None } else { Some(name_input.value()) };
        pd.filters.cpu_min = cpu_input.value().parse().ok();
        pd.filters.memory_min = mem_input.value().parse().ok();
        
        pd.apply_filters();
        wind.hide();
        
        let _ = display_process_info_gui_filtered(pd.processes.clone());
    });

    while app.wait() {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}

// Add this helper function to display filtered results
fn display_process_info_gui_filtered(processes: Vec<ProcessInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(800, 600)
        .with_label("Filtered Process Information");
    wind.set_color(Color::from_rgb(245, 245, 245));

    let mut table = Table::default()
        .with_size(780, 580)
        .with_pos(10, 10);
    table.set_rows(processes.len() as i32 + 1);
    table.set_cols(6);
    table.set_col_header(true);
    table.set_col_width_all(130);
    table.set_row_height_all(25);

    table.draw_cell(move |t, mut ctx, row, col, x, y, w, h| {
        match ctx {
            TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
            TableContext::ColHeader => {
                let headers = ["PID", "Name", "Status", "CPU (%)", "Memory (KB)", "Core"];
                if let Some(header) = headers.get(col as usize) {
                    draw_header(t, &mut ctx, x, y, w, h, header);
                }
            },
            TableContext::Cell => {
                if let Some(process) = processes.get((row as usize) - 1) {
                    let value = match col {
                        0 => process.pid.to_string(),
                        1 => process.name.clone(),
                        2 => process.status.clone(),
                        3 => format!("{:.2}", process.cpu_usage),
                        4 => process.memory.to_string(),
                        5 => process.core.map_or("N/A".to_string(), |c| c.to_string()),
                        _ => String::new(),
                    };
                    draw_cell(t, &mut ctx, x, y, w, h, &value);
                }
            },
            _ => (),
        }
    });

    wind.end();
    wind.show();

    while app.wait() {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}



use std::{
    fs,
    io::{self, stdin, stdout, BufRead, Write},
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use crossterm::{
    execute, terminal::{Clear, ClearType},
    cursor,
};
use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt, ProcessStatus};
use systemstat::{System as StatSystem, Platform};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Row as TuiRow, Table as TuiTable, TableState, Paragraph, Wrap},
    Terminal,
};
use termion::{
    raw::IntoRawMode,
    input::{MouseTerminal, TermRead},
    screen::AlternateScreen,
    event::Key,
};

use fltk::{
    app::{self, App},
    enums::Color,
    draw::{draw_line, draw_text, set_draw_color, draw_rectf},
    frame::Frame,
    window::Window,
};

use std::sync::mpsc::{self, Sender};


use fltk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;


type CpuData = Arc<Mutex<Vec<Vec<f32>>>>; 


pub fn next(state: &mut TableState, items: &Vec<TuiRow>) {
    let i = match state.selected() {
        Some(i) => {
            if i >= items.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}


pub fn previous(state: &mut TableState, items: &Vec<TuiRow>) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                items.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}


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


fn update_table<'a>(system: &'a System, _stat_sys: &'a StatSystem) -> (Vec<TuiRow<'a>>, Vec<String>){
    let mut rows = Vec::new();
    let mut pids = Vec::new();
    // Header row
    rows.push(TuiRow::new(vec![
        Span::styled("PID", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("NAME", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("STATUS", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("CPU %", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("MEMORY", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("CORE", Style::default().add_modifier(Modifier::BOLD)),
    ]));


    for (pid, process) in system.processes() {
        rows.push(TuiRow::new(vec![
            Span::raw(pid.to_string()),
            Span::raw(process.name().to_string()),
            Span::raw(format!("{:?}", process.status())),
            Span::raw(format!("{:.2}", process.cpu_usage())),
            Span::raw((process.memory() / 1024).to_string()), // Memory in MB
            Span::raw(
                get_cpu_core_for_pid(process.pid())
                    .map(|core| core.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        ]));
        pids.push(pid.to_string());
    }

    (rows,pids)
}


fn update_system_stats<'a>(system: &'a System, stat_sys: &'a StatSystem) -> Vec<TuiRow<'a>> {
    let mut rows = Vec::new();

    let loadavg = stat_sys.load_average().ok();
    let (one, five, fifteen) = if let Some(load) = loadavg {
        (load.one, load.five, load.fifteen)
    } else {
        (0.0, 0.0, 0.0)
    };

    let mut total_tasks = 0;
    let mut running = 0;
    let mut sleeping = 0;
    let mut stopped = 0;
    let mut zombie = 0;


    for process in system.processes() {
        let proc = process.1;
        total_tasks += 1;
        match proc.status() {
            ProcessStatus::Run => running += 1,
            ProcessStatus::Sleep => sleeping += 1,
            ProcessStatus::Stop => stopped += 1,
            ProcessStatus::Zombie => zombie += 1,
            _ => {}
        }
    }

    
    rows.push(TuiRow::new(vec![
        Span::styled(
            format!("Load Avg: {:.2}, {:.2}, {:.2}", one, five, fifteen),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Total Tasks: {}", total_tasks),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Running: {}", running),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Sleeping: {}", sleeping),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Stopped: {}", stopped),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Zombie: {}", zombie),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));

    rows
}


fn options_table<'a>() -> Paragraph<'a> {
    let options_text = vec![Spans::from(vec![
        Span::styled("q: Quit", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  |  "),
        Span::styled("Up/Down: Navigate", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  |  "),
        Span::styled("t: Process Tree", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  |  "),
        Span::styled("n: Change Niceness", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(" | k: Kill Process", Style::default().add_modifier(Modifier::BOLD)),
    ])];

    Paragraph::new(options_text).block(Block::default().borders(Borders::NONE))
}


fn get_cpu_usages(system: &System) -> Vec<f32> {
    system.processors().iter().map(|cpu| cpu.cpu_usage()).collect()
}


fn plot_cpu_usage(cpu_data: CpuData, num_cores: usize) {

    let app = App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 1000, 700, "CPU Utilization");

    let frame = Rc::new(RefCell::new(Frame::new(0, 0, 1000, 700, "")));

    wind.end();
    wind.show();

   
    let frame_clone = Rc::clone(&frame);
   
    app::add_timeout3(3.0, move |t| {
    
        frame_clone.borrow_mut().redraw();
    

        app::repeat_timeout3(3.0, t);
    });
    
   
    let frame_clone_draw = Rc::clone(&frame);
    frame.borrow_mut().draw(move |f| {
        let data = cpu_data.lock().unwrap();
        let width = f.width();
        let height = f.height();
        let margin = 50;
        let plot_width = width - 3* margin;
        let plot_height = height - 3 * margin;
     
        // Clear the background
        set_draw_color(Color::Black);
        fltk::draw::draw_rectf(0, 0, width, height);

        // Draw x-axis
        set_draw_color(Color::White);
        draw_line(margin, height - margin, width - margin, height - margin); // X-axis

        // Get the latest CPU usage data
        let latest_data = data.last().cloned().unwrap_or_else(|| vec![0.0; num_cores]);

        // Define bar properties
        let bar_spacing = 10;
        let total_bar_width = plot_width - (num_cores as i32 - 1) * bar_spacing;
        let bar_width = total_bar_width / num_cores as i32;

        // Adjust bar_width if it's too small or negative
        let bar_width = if bar_width > 0 { bar_width } else { 1 };

        let max_cpu = 100.0; // CPU usage percentage

        // Loop over cores to draw bars
        for (i, &cpu_usage) in latest_data.iter().enumerate() {

            // Clamp CPU usage to 0-100%
            let cpu_usage = if cpu_usage > max_cpu {
                max_cpu
            } else if cpu_usage < 0.0 {
                0.0
            } else {
                cpu_usage
            };

            set_draw_color(Color::Green);

            // Calculate bar dimensions
            let x = margin + i as i32 * (bar_width + bar_spacing);
            let bar_height = ((cpu_usage / max_cpu) * plot_height as f32) as i32;
            let y = height - margin - bar_height;

            // Draw the bar
            draw_rectf(x, y, bar_width, bar_height);

            // Draw the usage percentage over the bar
            let usage_text = format!("{:.1}%", cpu_usage);
            set_draw_color(Color::White);
            // Calculate text position (centered)
            let text_x = x + bar_width / 2 - (usage_text.len() as i32 * 3) / 2;
            let text_y = y - 10; // 10 pixels above the bar
            draw_text(&usage_text, text_x, text_y);

            // Label the bar from underneath with core number
            let core_label = format!("C {}", i);
            // Calculate label position (centered)
            let label_x = x + bar_width / 2 - (core_label.len() as i32 * 3) / 2;
            let label_y = height - margin + 20; // 20 pixels below the x-axis
            draw_text(&core_label, label_x, label_y);
        }
      
    
    });

    app.run().unwrap();
}

pub fn display_process_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut sysinfo = System::new_all();
    let stat_sys = StatSystem::new();
    let update_interval = Duration::from_secs(5);
    sysinfo.refresh_all();
    thread::sleep(Duration::from_secs(2));
    // Shared CPU data structure
    let cpu_data: CpuData = Arc::new(Mutex::new(Vec::new()));
    let cpu_data_clone = Arc::clone(&cpu_data);

    // Determine the number of CPU cores
    let num_cores = sysinfo.processors().len();

    // Start the plotting thread
    std::thread::spawn(move || {
        plot_cpu_usage(cpu_data_clone, num_cores);
    });

    // Setup terminal UI
    let stdout = stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to capture keyboard events
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys() {
            if let Ok(key) = key {
                if tx.send(key).is_err() {
                    break;
                }
            }
        }
    });

    let mut table_state = TableState::default();
    table_state.select(Some(0));
    let mut show_tree = false;
    let mut tree_output = Vec::new(); // Store process tree lines
    let mut tree_state = TableState::default(); // State for scrolling in process tree
    tree_state.select(Some(0)); // Initially select the first line

    loop {
        sysinfo.refresh_all(); // Refresh all system information

        // Update shared CPU data
        {
            let mut data = cpu_data.lock().unwrap();
            let cpu_usages = get_cpu_usages(&sysinfo);
            data.push(cpu_usages);
            // Limit the data to the latest entry
            if data.len() > 1 {
                data.remove(0);
            }
        }

        let stats_rows = update_system_stats(&sysinfo, &stat_sys);
        let (process_rows, process_pids) = update_table(&sysinfo, &stat_sys);

        terminal.draw(|f| {
            let size = f.size();

            if show_tree {
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(90), Constraint::Length(1)])
                    .split(size);

                let start_index = tree_state.selected().unwrap_or(0);
                let end_index = (start_index + chunks[0].height as usize).min(tree_output.len());
                let visible_lines = &tree_output[start_index..end_index];

                let tree_block = Paragraph::new(visible_lines.join("\n"))
                    .block(Block::default().borders(Borders::ALL).title("Process Tree"))
                    .wrap(Wrap { trim: true });
                f.render_widget(tree_block, chunks[0]);

                let options_block = options_table();
                f.render_widget(options_block, chunks[1]);
            } else {
                let chunks = Layout::default()
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Percentage(85),
                        Constraint::Length(1),
                    ])
                    .split(size);

                let stats_table = TuiTable::new(stats_rows)
                    .block(Block::default().borders(Borders::ALL).title("System Stats"))
                    .widths(&[
                        Constraint::Length(30), // Load Avg
                        Constraint::Length(20), // Total Tasks
                        Constraint::Length(15), // Running
                        Constraint::Length(15), // Sleeping
                        Constraint::Length(15), // Stopped
                        Constraint::Length(15), // Zombie
                    ]);
                f.render_widget(stats_table, chunks[0]);

                let process_table = TuiTable::new(process_rows.clone())
                    .block(Block::default().borders(Borders::ALL).title("Process Info"))
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .widths(&[
                        Constraint::Length(20), // PID column
                        Constraint::Length(40), // NAME column
                        Constraint::Length(20), // STATUS column
                        Constraint::Length(20), // CPU % column
                        Constraint::Length(20), // MEMORY column
                        Constraint::Length(20), // CORE column
                    ]);
                f.render_stateful_widget(process_table, chunks[1], &mut table_state);

                let options_block = options_table();
                f.render_widget(options_block, chunks[2]);
            }
        })?;

        // Handle user input
        if let Ok(key) = rx.recv_timeout(update_interval) {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    if show_tree {
                        let selected = tree_state.selected().unwrap_or(0);
                        if selected < tree_output.len() - 1 {
                            tree_state.select(Some(selected + 1));
                        }
                    } else {
                        next(&mut table_state, &process_rows);
                    }
                }
                Key::Up => {
                    if show_tree {
                        let selected = tree_state.selected().unwrap_or(0);
                        if selected > 0 {
                            tree_state.select(Some(selected - 1));
                        }
                    } else {
                        previous(&mut table_state, &process_rows);
                    }
                }
                Key::Char('t') => {
                    if show_tree {
                        show_tree = false;
                    } else {
                        let output = Command::new("pstree")
                            .arg("-p")
                            .output()
                            .expect("Failed to execute pstree command");

                        if output.status.success() {
                            tree_output = String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .map(|line| line.to_string())
                                .collect();
                            tree_state.select(Some(0)); // Reset scroll to the top
                            show_tree = true;
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Error executing pstree: {}", stderr);
                        }
                    }
                }
                Key::Char('n') => {
                    println!("Enter the PID of the process to change niceness:");
                    let stdin = stdin();
                    let mut input = String::new();
                    std::io::BufRead::read_line(&mut stdin.lock(), &mut input)?; 
                    let pid = input.trim().parse::<i32>();

                    if let Ok(pid) = pid {
                        println!("Enter the new niceness value (-20 to 19):");
                        input.clear();
                        std::io::BufRead::read_line(&mut stdin.lock(), &mut input)?; 
                        let niceness = input.trim().parse::<i32>();

                        if let Ok(niceness) = niceness {
                            println!("Attempting to change niceness for PID {} to {}", pid, niceness);

                            let output = Command::new("sudo")
                                .arg("renice")
                                .arg(niceness.to_string())
                                .arg("-p")
                                .arg(pid.to_string())
                                .output();

                            match output {
                                Ok(output) if output.status.success() => {
                                    println!(
                                        "Successfully changed niceness for PID {}. Output:\n{}",
                                        pid,
                                        String::from_utf8_lossy(&output.stdout)
                                    );
                                }
                                Ok(output) => {
                                    let error = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Failed to change niceness. Error:\n{}", error);
                                }
                                Err(err) => {
                                    eprintln!("Error executing renice command: {}", err);
                                }
                            }
                        } else {
                            eprintln!("Invalid niceness value. Must be between -20 and 19.");
                        }
                    } else {
                        eprintln!("Invalid PID. Please enter a valid numeric PID.");
                    }
                }
                Key::Char('k') => {
                    if let Some(selected) = table_state.selected() {
                        if let Some(pid_str) = process_pids.get(selected) {
                            if let Ok(pid) = pid_str.parse::<i32>() {
                                let output = Command::new("kill")
                                    .arg("-9")
                                    .arg(pid.to_string())
                                    .output();

                                match output {
                                    Ok(output) if output.status.success() => {
                                        println!("Successfully killed process with PID {}", pid);
                                    }
                                    Ok(output) => {
                                        eprintln!(
                                            "Failed to kill process: {}",
                                            String::from_utf8_lossy(&output.stderr)
                                        );
                                    }
                                    Err(err) => {
                                        eprintln!("Error executing kill command: {}", err);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}


// use std::{
//     fs,
//     io::{self, stdin, stdout, BufRead, Write},
//     process::Command,
//     sync::{Arc, Mutex},
//     thread,
//     time::Duration,
// };
// use crossterm::{
//     execute, terminal::{Clear, ClearType},
//     cursor,
// };
// use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt, ProcessStatus};
// use systemstat::{System as StatSystem, Platform};
// use tui::{
//     backend::TermionBackend,
//     layout::{Layout, Constraint},
//     style::{Modifier, Style},
//     text::{Span, Spans},
//     widgets::{Block, Borders, Row as TuiRow, Table as TuiTable, TableState, Paragraph, Wrap},
//     Terminal,
// };
// use termion::{
//     raw::IntoRawMode,
//     input::{MouseTerminal, TermRead},
//     screen::AlternateScreen,
//     event::Key,
// };

// use fltk::{
//     app::{self, App},
//     enums::Color,
//     draw::{draw_line, draw_text, set_draw_color, draw_rectf},
//     frame::Frame,
//     window::Window,
// };

// use std::sync::mpsc::{self, Sender};


// use fltk::prelude::*;
// use std::rc::Rc;
// use std::cell::RefCell;


// type CpuData = Arc<Mutex<Vec<Vec<f32>>>>; 


// fn get_cpu_usages(system: &System) -> Vec<f32> {
//     system.processors().iter().map(|cpu| cpu.cpu_usage()).collect()
// }



// fn plot_cpu_usage(cpu_data: CpuData, num_cores: usize) {
//     // Calculate dynamic window width based on the number of cores
//     let base_width = 800;
//     let min_width_per_core = 50; // Minimum width allocated for each core
//     let window_width = base_width.max(num_cores * min_width_per_core);

//     let app = App::default().with_scheme(app::Scheme::Gleam);
//     let mut wind = Window::new(100, 100, window_width as i32, 600, "CPU Utilization");

//     let frame = Rc::new(RefCell::new(Frame::new(0, 0, window_width as i32, 600, "")));

//     wind.end();
//     wind.show();

//     let frame_clone = Rc::clone(&frame);

//     app::add_timeout3(3.0, move |t| {
//         frame_clone.borrow_mut().redraw();
//         app::repeat_timeout3(3.0, t);
//     });

//     let frame_clone_draw = Rc::clone(&frame);
//     frame.borrow_mut().draw(move |f| {
//         let data = cpu_data.lock().unwrap();
//         let width = f.width();
//         let height = f.height();
//         let margin = 50;
//         let plot_width = width - 2 * margin;
//         let plot_height = height - 2 * margin;

//         // Clear the background
//         set_draw_color(Color::Black);
//         fltk::draw::draw_rectf(0, 0, width, height);

//         // Draw x-axis
//         set_draw_color(Color::White);
//         draw_line(margin, height - margin, width - margin, height - margin); // X-axis

//         // Get the latest CPU usage data
//         let latest_data = data.last().cloned().unwrap_or_else(|| vec![0.0; num_cores]);

//         // Define bar properties
//         let bar_spacing = 10;
//         let total_bar_width = plot_width - (num_cores as i32 - 1) * bar_spacing;
//         let bar_width = total_bar_width / num_cores as i32;

//         // Adjust bar_width if it's too small or negative
//         let bar_width = if bar_width > 0 { bar_width } else { 1 };

//         let max_cpu = 100.0; // CPU usage percentage

//         // Loop over cores to draw bars
//         for (i, &cpu_usage) in latest_data.iter().enumerate() {
//             // Clamp CPU usage to 0-100%
//             let cpu_usage = cpu_usage.clamp(0.0, max_cpu);

//             set_draw_color(Color::Green);

//             // Calculate bar dimensions
//             let x = margin + i as i32 * (bar_width + bar_spacing);
//             let bar_height = ((cpu_usage / max_cpu) * plot_height as f32) as i32;
//             let y = height - margin - bar_height;

//             // Draw the bar
//             draw_rectf(x, y, bar_width, bar_height);

//             // Draw the usage percentage over the bar
//             let usage_text = format!("{:.1}%", cpu_usage);
//             set_draw_color(Color::White);
//             // Calculate text position (centered)
//             let text_x = x + bar_width / 2 - (usage_text.len() as i32 * 3) / 2;
//             let text_y = y - 10; // 10 pixels above the bar
//             draw_text(&usage_text, text_x, text_y);

//             // Label the bar from underneath with core number
//             let core_label = format!("Core {}", i);
//             // Calculate label position (centered)
//             let label_x = x + bar_width / 2 - (core_label.len() as i32 * 3) / 2;
//             let label_y = height - margin + 20; // 20 pixels below the x-axis
//             draw_text(&core_label, label_x, label_y);
//         }
//     });

//     app.run().unwrap();
// }

// pub fn next(state: &mut TableState, items: &Vec<TuiRow>) {
//     let i = match state.selected() {
//         Some(i) => {
//             if i >= items.len() - 1 {
//                 0
//             } else {
//                 i + 1
//             }
//         }
//         None => 0,
//     };
//     state.select(Some(i));
// }

// pub fn previous(state: &mut TableState, items: &Vec<TuiRow>) {
//     let i = match state.selected() {
//         Some(i) => {
//             if i == 0 {
//                 items.len() - 1
//             } else {
//                 i - 1
//             }
//         }
//         None => 0,
//     };
//     state.select(Some(i));
// }

// fn update_table<'a>(system: &'a System) -> (Vec<TuiRow<'a>>, Vec<String>) {
//     let mut rows = Vec::new();
//     let mut pids = Vec::new();

//     rows.push(TuiRow::new(vec![
//         Span::styled("PID", Style::default().add_modifier(Modifier::BOLD)),
//         Span::styled("NAME", Style::default().add_modifier(Modifier::BOLD)),
//         Span::styled("STATUS", Style::default().add_modifier(Modifier::BOLD)),
//         Span::styled("CPU %", Style::default().add_modifier(Modifier::BOLD)),
//         Span::styled("MEMORY", Style::default().add_modifier(Modifier::BOLD)),
//     ]));

//     for (pid, process) in system.processes() {
//         rows.push(TuiRow::new(vec![
//             Span::raw(pid.to_string()),
//             Span::raw(process.name().to_string()),
//             Span::raw(format!("{:?}", process.status())),
//             Span::raw(format!("{:.2}", process.cpu_usage())),
//             Span::raw((process.memory() / 1024).to_string()), // Memory in MB
//         ]));
//         pids.push(pid.to_string());
//     }

//     (rows, pids)
// }

// fn update_system_stats<'a>(system: &'a System, stat_sys: &'a StatSystem) -> Vec<TuiRow<'a>> {
//     let mut rows = Vec::new();

//     let loadavg = stat_sys.load_average().ok();
//     let (one, five, fifteen) = if let Some(load) = loadavg {
//         (load.one, load.five, load.fifteen)
//     } else {
//         (0.0, 0.0, 0.0)
//     };

//     let mut total_tasks = 0;
//     let mut running = 0;
//     let mut sleeping = 0;
//     let mut stopped = 0;
//     let mut zombie = 0;

//     for process in system.processes() {
//         let proc = process.1;
//         total_tasks += 1;
//         match proc.status() {
//             sysinfo::ProcessStatus::Run => running += 1,
//             sysinfo::ProcessStatus::Sleep => sleeping += 1,
//             sysinfo::ProcessStatus::Stop => stopped += 1,
//             sysinfo::ProcessStatus::Zombie => zombie += 1,
//             _ => {}
//         }
//     }

//     rows.push(TuiRow::new(vec![
//         Span::styled(
//             format!("Load Avg: {:.2}, {:.2}, {:.2}", one, five, fifteen),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//         Span::styled(
//             format!("Total Tasks: {}", total_tasks),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//         Span::styled(
//             format!("Running: {}", running),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//         Span::styled(
//             format!("Sleeping: {}", sleeping),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//         Span::styled(
//             format!("Stopped: {}", stopped),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//         Span::styled(
//             format!("Zombie: {}", zombie),
//             Style::default().add_modifier(Modifier::BOLD),
//         ),
//     ]));

//     rows
// }

// fn options_table<'a>() -> Paragraph<'a> {
//     let options_text = vec![Spans::from(vec![
//         Span::styled("q: Quit", Style::default().add_modifier(Modifier::BOLD)),
//         Span::raw("  |  "),
//         Span::styled("Up/Down: Navigate", Style::default().add_modifier(Modifier::BOLD)),
//         Span::raw("  |  "),
//         Span::styled("t: Process Tree", Style::default().add_modifier(Modifier::BOLD)),
//         Span::raw("  |  "),
//         Span::styled("k: Kill Process", Style::default().add_modifier(Modifier::BOLD)),
//     ])];

//     Paragraph::new(options_text).block(Block::default().borders(Borders::NONE))
// }

// pub fn display_process_info() -> Result<(), Box<dyn std::error::Error>> {
//     let mut sysinfo = System::new_all();
//     let stat_sys = StatSystem::new();
//     sysinfo.refresh_all();
//     thread::sleep(Duration::from_secs(2));
//         // Shared CPU data structure
//         let cpu_data: CpuData = Arc::new(Mutex::new(Vec::new()));
//         let cpu_data_clone = Arc::clone(&cpu_data);
    
//         // Determine the number of CPU cores
//         let num_cores = sysinfo.processors().len();
    
//         // Start the plotting thread
//         std::thread::spawn(move || {
//             plot_cpu_usage(cpu_data_clone, num_cores);
//         });

//     let stdout = stdout().into_raw_mode()?;
//     let stdout = MouseTerminal::from(stdout);
//     let stdout = AlternateScreen::from(stdout);
//     let backend = TermionBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let stdin = std::io::stdin();
//         for key in stdin.keys() {
//             if let Ok(key) = key {
//                 if tx.send(key).is_err() {
//                     break;
//                 }
//             }
//         }
//     });

//     let mut table_state = TableState::default();
//     table_state.select(Some(0));

//     let mut show_tree = false;
//     let mut tree_output = Vec::new();
//     let mut tree_state = TableState::default();
//     tree_state.select(Some(0));

//     loop {
//         sysinfo.refresh_all();

//         {
//             let mut data = cpu_data.lock().unwrap();
//             let cpu_usages = get_cpu_usages(&sysinfo);
//             data.push(cpu_usages);
//             // Limit the data to the latest entry
//             if data.len() > 1 {
//                 data.remove(0);
//             }
//         }
//         let stats_rows = update_system_stats(&sysinfo, &stat_sys);
//         let (process_rows, process_pids) = update_table(&sysinfo);

//         terminal.draw(|f| {
//             let size = f.size();
//             let chunks = Layout::default()
//                 .constraints([
//                     Constraint::Length(3),  // System Stats
//                     Constraint::Percentage(85), // Main Content
//                     Constraint::Length(2),  // Options
//                 ])
//                 .split(size);

//             let stats_table = TuiTable::new(stats_rows)
//                 .block(Block::default().borders(Borders::ALL).title("System Stats"))
//                 .widths(&[
//                     Constraint::Length(25),
//                     Constraint::Length(25),
//                     Constraint::Length(20),
//                     Constraint::Length(20),
//                     Constraint::Length(20),
//                     Constraint::Length(20),
//                 ]);

//             f.render_widget(stats_table, chunks[0]);

//             if show_tree {
//                 let start_index = tree_state.selected().unwrap_or(0);
//                 let end_index = (start_index + chunks[1].height as usize).min(tree_output.len());
//                 let visible_lines = &tree_output[start_index..end_index];

//                 let tree_block = Paragraph::new(visible_lines.join("\n"))
//                     .block(Block::default().borders(Borders::ALL).title("Process Tree"));

//                 f.render_widget(tree_block, chunks[1]);
//             } else {
//                 let process_table = TuiTable::new(process_rows.clone())
//                     .block(Block::default().borders(Borders::ALL).title("Processes"))
//                     .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
//                     .widths(&[
//                         Constraint::Length(10),
//                         Constraint::Length(25),
//                         Constraint::Length(20),
//                         Constraint::Length(15),
//                         Constraint::Length(15),
//                     ]);

//                 f.render_stateful_widget(process_table, chunks[1], &mut table_state);
//             }

//             let options_block = options_table();
//             f.render_widget(options_block, chunks[2]);
//         })?;

//         if let Ok(key) = rx.recv_timeout(Duration::from_millis(250)) {
//             match key {
//                 Key::Char('q') => break,
//                 Key::Down => {
//                     if show_tree {
//                         let selected = tree_state.selected().unwrap_or(0);
//                         if selected < tree_output.len() - 1 {
//                             tree_state.select(Some(selected + 1));
//                         }
//                     } else {
//                         next(&mut table_state, &process_rows);
//                     }
//                 }
//                 Key::Up => {
//                     if show_tree {
//                         let selected = tree_state.selected().unwrap_or(0);
//                         if selected > 0 {
//                             tree_state.select(Some(selected - 1));
//                         }
//                     } else {
//                         previous(&mut table_state, &process_rows);
//                     }
//                 }
//                 Key::Char('t') => {
//                     if show_tree {
//                         show_tree = false;
//                     } else {
//                         let output = Command::new("pstree")
//                             .arg("-p")
//                             .output();

//                         match output {
//                             Ok(output) if output.status.success() => {
//                                 tree_output = String::from_utf8_lossy(&output.stdout)
//                                     .lines()
//                                     .map(String::from)
//                                     .collect();
//                                 tree_state.select(Some(0));
//                                 show_tree = true;
//                             }
//                             Ok(output) => {
//                                 eprintln!(
//                                     "Error displaying process tree: {}",
//                                     String::from_utf8_lossy(&output.stderr)
//                                 );
//                             }
//                             Err(err) => {
//                                 eprintln!("Failed to execute pstree command: {}", err);
//                             }
//                         }
//                     }
//                 }
//                 Key::Char('k') => {
//                     if let Some(selected) = table_state.selected() {
//                         if let Some(pid_str) = process_pids.get(selected) {
//                             if let Ok(pid) = pid_str.parse::<i32>() {
//                                 let output = Command::new("kill")
//                                     .arg("-9")
//                                     .arg(pid.to_string())
//                                     .output();

//                                 match output {
//                                     Ok(output) if output.status.success() => {
//                                         println!("Successfully killed process with PID {}", pid);
//                                     }
//                                     Ok(output) => {
//                                         eprintln!(
//                                             "Failed to kill process: {}",
//                                             String::from_utf8_lossy(&output.stderr)
//                                         );
//                                     }
//                                     Err(err) => {
//                                         eprintln!("Error executing kill command: {}", err);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

//     Ok(())
// }


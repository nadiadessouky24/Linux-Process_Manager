mod load_avg; 
mod input;
mod syscalls;
mod common;
mod ctrlc_handler;
mod zombie_processes; 
mod process_tree;
mod threshold_monitor;
mod handling_filter;
mod display_filtered;
mod filtering;
mod filter_gui;
mod gui_display;
mod cli_display;

use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    dialog,
};
use ctrlc_handler::{setup_ctrlc_handler, exiting_loop, RUNNING};
use crate::common::Ordering;
use load_avg::{display_load_avg_gui, display_load_avg_cli};
use zombie_processes::{display_zombie_processes_gui, display_zombie_processes_cli};
use syscalls::{syscalls_gui, syscalls_cli};
use process_tree::display_process_tree_gui;
use threshold_monitor::{display_threshold_monitor_gui, set_thresholds_cli, cleanup_monitor, start_threshold_monitor};
use handling_filter::handle_filter_process;
use gui_display::{display_process_info_gui, display_filter_gui};
use cli_display::display_process_info;

fn run_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    
    // Create FLTK channel for warnings
    let (sender, receiver) = app::channel::<String>();
    
    start_threshold_monitor(2000.0, Some(sender.clone()));
    
    let mut wind = Window::default()
        .with_size(400, 650)
        .with_label("System Monitor")
        .center_screen();
    
    let mut title = Frame::default()
        .with_size(380, 40)
        .with_pos(10, 10)
        .with_label("Welcome to Linux Process Manager");
    title.set_label_color(fltk::enums::Color::from_rgb(51, 122, 183));
    title.set_label_size(16);
    
    // Make all buttons the same size
    let button_width = 380;
    let button_height = 50;
    let button_spacing = 10;
    let button_x = 10;
    let mut button_y = 60;
    
    // Create all buttons with the same size and style
    let mut process_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Process Information");
    button_y += button_height + button_spacing;
    
    let mut loadavg_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Load Average");
    button_y += button_height + button_spacing;
    
    let mut syscalls_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("System Calls");
    button_y += button_height + button_spacing;
    
    let mut zombie_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Zombie Processes");
    button_y += button_height + button_spacing;
    
    let mut proctree_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Process Tree Viewer");
    button_y += button_height + button_spacing;
    
    let mut threshold_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Resource Thresholds");
    button_y += button_height + button_spacing;
    
    let mut filter_btn = Button::default()
        .with_size(button_width, button_height)
        .with_pos(button_x, button_y)
        .with_label("Filter Processes");
    
    // Apply the same style to all buttons
    let mut buttons = [
        &mut process_btn, 
        &mut loadavg_btn, 
        &mut syscalls_btn, 
        &mut zombie_btn, 
        &mut proctree_btn, 
        &mut threshold_btn,
        &mut filter_btn
    ];

    for btn in buttons.iter_mut() {
        btn.set_color(fltk::enums::Color::from_rgb(91, 192, 222));
        btn.set_label_color(fltk::enums::Color::White);
        btn.set_label_size(14);
    }

    wind.end();
    wind.show();

    // Update the filter button callback
    filter_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_filter_gui();
    });

    // Set up callbacks
    process_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_process_info_gui();
    });

    loadavg_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_load_avg_gui();
    });

    syscalls_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = syscalls_gui();
    });

    zombie_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_zombie_processes_gui();
    });

    proctree_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_process_tree_gui();
    });

    threshold_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_threshold_monitor_gui();
    });

    // Main event loop
    while app.wait() {
        if let Some(warning) = receiver.recv() {
            println!("Received warning in GUI: {}", warning);
            dialog::alert_default(&warning);
            app.redraw();
        }
        
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }

    cleanup_monitor();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_ctrlc_handler()?;
    
    println!("\nGUI/CLI: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    match input.trim().to_uppercase().as_str() {
        "GUI" => run_gui()?,
        "CLI" => display_process_info()?,
        _ => println!("Invalid input. Please enter either 'GUI' or 'CLI'"),
    }
    
    Ok(())
}
mod load_avg; 
mod process_display;
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

use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    dialog,
};
use ctrlc_handler::{exiting_loop, RUNNING};
use crate::common::Ordering;
use load_avg::{display_load_avg_gui, display_load_avg_cli};
use zombie_processes::{display_zombie_processes_gui, display_zombie_processes_cli};
use syscalls::{syscalls_gui, syscalls_cli};
use process_tree::display_process_tree_gui;
use threshold_monitor::{display_threshold_monitor_gui, set_thresholds_cli, cleanup_monitor, start_threshold_monitor};
use handling_filter::handle_filter_process;
use process_display::{display_process_info_gui, display_process_info_cli, display_filter_gui};

fn run_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    
    // Create FLTK channel for warnings
    let (sender, receiver) = app::channel::<String>();
    
    start_threshold_monitor(80.0, Some(sender.clone()));
    
    let mut wind = Window::default()
        .with_size(400, 600)
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

    // Set up callbacks
    filter_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_filter_gui();
    });

    threshold_btn.set_callback(move |_| {
        RUNNING.store(true, Ordering::SeqCst);
        let _ = display_threshold_monitor_gui();
    });

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

    // Main event loop
    while app.wait() {
        if let Some(warning) = receiver.recv() {
            println!("Received warning in GUI: {}", warning);
            dialog::alert_default(&warning);
            app.redraw();  // Ensure the GUI updates
        }
        
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }

    cleanup_monitor();
    Ok(())
}

fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let input = input::get_user_input("\n To Display Process information enter 'display', \n To display filtered process enter 'filter',\n To display load average enter 'loadavg' \n To run system calls enter 'command' \n To Display Zombie Processes enter 'zombies' \n To view process tree enter 'tree' \n To set resource thresholds enter 'threshold' \n To exit enter 'exit': ");

        match input.as_str() {
            "display" => {
                RUNNING.store(true, Ordering::SeqCst); 
                display_process_info_cli()?;
            }
            "filter" => {
                handle_filter_process()?;
            }
            "loadavg" => {
                RUNNING.store(true, Ordering::SeqCst); 
                display_load_avg_cli()?; 
            }
            "command" => {
                syscalls_cli();
            }
            "zombies" => {
                display_zombie_processes_cli();
            }
            "tree" => {
                println!("Process tree viewer is only available in GUI mode");
            }
            "threshold" => {
                RUNNING.store(true, Ordering::SeqCst);
                set_thresholds_cli()?;
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Unknown command, try again.");
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    exiting_loop();
    
    println!("Welcome to Linux Process Manager!");
    println!("Please choose your interface:");
    println!("1. Type 'gui' for Graphical User Interface");
    println!("2. Type 'cli' for Command Line Interface");
    
    let choice = input::get_user_input("Enter your choice (gui/cli): ");
    
    match choice.to_lowercase().as_str() {
        "gui" => run_gui()?,
        "cli" => {
            // Start threshold monitor without warnings for CLI
            start_threshold_monitor(80.0, None);
            run_cli()?
        },
        _ => {
            println!("Invalid choice. Please run the program again and choose either 'gui' or 'cli'.");
            return Ok(());
        }
    }
    
    cleanup_monitor();
    Ok(())
}

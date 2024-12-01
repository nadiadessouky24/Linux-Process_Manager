use fltk::{
    app,
    prelude::*,
    window::Window,
    input::Input,
    button::Button,
    dialog,
    enums::Color,
};
use sysinfo::{System, SystemExt, ProcessorExt, ProcessExt};
use std::sync::atomic::{AtomicI32, Ordering, AtomicBool, AtomicI64};
use std::time::Duration;
use crate::ctrlc_handler::RUNNING;
use std::thread;
use fltk::app::Sender;
use chrono::Local;

// Changed from AtomicF32 to AtomicI32, will store values multiplied by 100 to maintain precision
static CPU_THRESHOLD: AtomicI32 = AtomicI32::new(8000); // Default 80.00%
static MEM_THRESHOLD: AtomicI32 = AtomicI32::new(8000); // Default 80.00%
static THRESHOLD: AtomicI64 = AtomicI64::new(8000); // Default 80.00%
static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn display_threshold_monitor_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    
    // Create a channel for warnings
    let (warning_sender, warning_receiver) = app::channel::<String>();
    
    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("Resource Threshold Settings");
    wind.set_color(Color::from_rgb(245, 245, 245));

    // Add a frame to display warnings
    let mut warning_display = fltk::text::TextDisplay::default()
        .with_size(480, 150)
        .with_pos(10, 240)
        .with_label("Warnings:");
    let mut warning_buffer = fltk::text::TextBuffer::default();
    warning_display.set_buffer(warning_buffer.clone());
    
    let mut cpu_input = Input::default()
        .with_size(150, 30)
        .with_pos(200, 50)
        .with_label("Threshold (%):");
    let current = THRESHOLD.load(Ordering::SeqCst) as f64 / 100.0;
    cpu_input.set_value(&format!("{:.1}", current));
    cpu_input.set_text_size(14);

    let mut save_btn = Button::default()
        .with_size(150, 40)
        .with_pos(200, 190)
        .with_label("Save Threshold");
    save_btn.set_color(Color::from_rgb(51, 122, 183));
    save_btn.set_label_color(Color::White);
    save_btn.set_label_size(16);

    wind.end();
    wind.show();

    let cpu_input_clone = cpu_input.clone();
    save_btn.set_callback(move |_| {
        if let Ok(value) = cpu_input_clone.value().parse::<f64>() {
            if value > 0.0 && value <= 100.0 {
                THRESHOLD.store((value * 100.0) as i64, Ordering::SeqCst);
                println!("Threshold updated to: {:.1}%", value);
                dialog::message(200, 200, "Threshold updated successfully!");
            } else {
                dialog::alert(200, 200, "Please enter a value between 0 and 100!");
            }
        } else {
            dialog::alert(200, 200, "Please enter a valid number!");
        }
    });

    // Start the threshold monitor with the current threshold
    let current_threshold = THRESHOLD.load(Ordering::SeqCst) as f64 / 100.0;
    start_threshold_monitor(current_threshold, Some(warning_sender.clone()));

    while app.wait() {
        if let Some(warning) = warning_receiver.recv() {
            let timestamp = Local::now().format("%H:%M:%S").to_string();
            warning_buffer.append(&format!("[{}] {}\n", timestamp, warning));
            warning_display.scroll(warning_display.count_lines(0, 
                warning_display.buffer().unwrap().length(), true), 0);
            dialog::message_default(&warning);
        }

        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }
    
    cleanup_monitor();
    Ok(())
}

pub fn start_threshold_monitor(default_threshold: f64, sender: Option<Sender<String>>) {
    THRESHOLD.store((default_threshold * 100.0) as i64, Ordering::SeqCst);
    MONITOR_RUNNING.store(true, Ordering::SeqCst);
    println!("Starting monitor with threshold: {:.1}%", default_threshold);

    if let Some(sender) = sender {
        thread::spawn(move || {
            let mut sys = System::new_all();
            
            while MONITOR_RUNNING.load(Ordering::SeqCst) {
                sys.refresh_all();
                sys.refresh_cpu();
                
                let threshold = THRESHOLD.load(Ordering::SeqCst) as f64 / 100.0;
                
                // Check individual process CPU usage
                for (pid, process) in sys.processes() {
                    let cpu_usage = process.cpu_usage() as f64;
                    
                    if cpu_usage >= threshold {
                        let warning = format!(
                            "WARNING: Process {} (PID: {}) CPU usage ({:.1}%) exceeds threshold of {:.1}%!",
                            process.name(), 
                            pid, 
                            cpu_usage,
                            threshold
                        );
                        let _ = sender.send(warning);
                        thread::sleep(Duration::from_secs(2));
                    }
                }
                
                // Check memory usage
                let total_mem = sys.total_memory() as f64;
                
                // Check individual process memory usage
                for (pid, process) in sys.processes() {
                    let proc_mem_usage = (process.memory() as f64 / total_mem) * 100.0;
                    
                    if proc_mem_usage >= threshold {
                        let warning = format!(
                            "WARNING: Process {} (PID: {}) Memory usage ({:.1}%) exceeds threshold of {:.1}%!",
                            process.name(), pid, proc_mem_usage, threshold
                        );
                        let _ = sender.send(warning);
                        thread::sleep(Duration::from_secs(2));
                    }
                }
                
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}

pub fn cleanup_monitor() {
    MONITOR_RUNNING.store(false, Ordering::SeqCst);
}
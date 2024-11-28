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

// Changed from AtomicF32 to AtomicI32, will store values multiplied by 100 to maintain precision
static CPU_THRESHOLD: AtomicI32 = AtomicI32::new(8000); // Default 80.00%
static MEM_THRESHOLD: AtomicI32 = AtomicI32::new(8000); // Default 80.00%
static THRESHOLD: AtomicI64 = AtomicI64::new(8000); // Default 80.00%
static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn display_threshold_monitor_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(500, 300)
        .with_label("Resource Threshold Settings");
    wind.set_color(Color::from_rgb(245, 245, 245));

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
                CPU_THRESHOLD.store((value * 100.0) as i32, Ordering::SeqCst);
                MEM_THRESHOLD.store((value * 100.0) as i32, Ordering::SeqCst);
                println!("Threshold updated to: {:.1}%", value);
                dialog::message(200, 200, "Threshold updated successfully!");
            } else {
                dialog::alert(200, 200, "Please enter a value between 0 and 100!");
            }
        } else {
            dialog::alert(200, 200, "Please enter a valid number!");
        }
    });

    while app.wait() {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}

pub fn set_thresholds_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter CPU usage threshold (%):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if let Ok(cpu) = input.trim().parse::<f32>() {
        if cpu > 0.0 && cpu <= 100.0 {
            CPU_THRESHOLD.store((cpu * 100.0) as i32, Ordering::Relaxed);
        }
    }

    println!("Enter memory usage threshold (%):");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    if let Ok(mem) = input.trim().parse::<f32>() {
        if mem > 0.0 && mem <= 100.0 {
            MEM_THRESHOLD.store((mem * 100.0) as i32, Ordering::Relaxed);
        }
    }

    let mut sys = System::new_all();
    while RUNNING.load(Ordering::SeqCst) {
        sys.refresh_all();
        
        let cpu_threshold = CPU_THRESHOLD.load(Ordering::Relaxed) as f32 / 100.0;
        let mem_threshold = MEM_THRESHOLD.load(Ordering::Relaxed) as f32 / 100.0;
        
        for (_, process) in sys.processes() {
            if process.cpu_usage() > cpu_threshold {
                println!(
                    "WARNING: Process {} (PID: {}) exceeded CPU threshold: {:.1}%",
                    process.name(), process.pid(), process.cpu_usage()
                );
            }
            
            let total_mem = sys.total_memory() as f32;
            let mem_usage = (process.memory() as f32 / total_mem) * 100.0;
            if mem_usage > mem_threshold {
                println!(
                    "WARNING: Process {} (PID: {}) exceeded memory threshold: {:.1}%",
                    process.name(), process.pid(), mem_usage
                );
            }
        }
        
        std::thread::sleep(Duration::from_secs(5));
    }
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
                let physical_cores = sys.physical_core_count().unwrap_or(1) as f64;
                
                // Check individual process CPU usage
                for (pid, process) in sys.processes() {
                    let cpu_usage = process.cpu_usage() as f64;
                    
                    if cpu_usage > threshold {
                        let warning = format!(
                            "WARNING: Process {} (PID: {}) CPU usage ({:.1}% of {} physical cores) exceeds threshold of {:.1}%!",
                            process.name(), 
                            pid, 
                            cpu_usage,
                            physical_cores as u32,
                            threshold
                        );
                        println!("Sending CPU warning to GUI");
                        let _ = sender.send(warning);
                        thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                }
                
                // Check memory usage
                let total_mem = sys.total_memory() as f64;
                let used_mem = sys.used_memory() as f64;
                let mem_usage = (used_mem / total_mem) * 100.0;
                
                // Check individual process memory usage
                for (pid, process) in sys.processes() {
                    let proc_mem_usage = (process.memory() as f64 / total_mem) * 100.0;
                    if proc_mem_usage > threshold {
                        let warning = format!(
                            "WARNING: Process {} (PID: {}) Memory usage ({:.1}%) exceeds threshold of {:.1}%!",
                            process.name(), pid, proc_mem_usage, threshold
                        );
                        println!("Sending Memory warning to GUI");
                        let _ = sender.send(warning);
                        // Add delay after sending warning to prevent spam
                        thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                }
                
                // Regular sleep between checks
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}

pub fn cleanup_monitor() {
    MONITOR_RUNNING.store(false, Ordering::SeqCst);
}

fn check_thresholds() {
    if let Ok(cpu_usage) = get_cpu_usage() {
        if cpu_usage > (THRESHOLD.load(Ordering::SeqCst) as f64 / 100.0) {
            println!("WARNING: CPU usage ({:.1}%) exceeds threshold!", cpu_usage);
        }
    }

    if let Ok(mem_usage) = get_memory_usage() {
        if mem_usage > (THRESHOLD.load(Ordering::SeqCst) as f64 / 100.0) {
            println!("WARNING: Memory usage ({:.1}%) exceeds threshold!", mem_usage);
        }
    }
}

// Helper functions to get system metrics
fn get_cpu_usage() -> Result<f64, std::io::Error> {
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(sys.global_processor_info().cpu_usage() as f64)
}

fn get_memory_usage() -> Result<f64, std::io::Error> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total = sys.total_memory() as f64;
    let used = sys.used_memory() as f64;
    Ok((used / total) * 100.0)
}
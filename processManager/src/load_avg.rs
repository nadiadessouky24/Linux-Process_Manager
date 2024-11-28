use systemstat::{System as StatSystem, Platform};
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::io::Write;
use crate::ctrlc_handler::RUNNING;
use fltk::{
    app,
    prelude::*,
    window::Window,
    frame::Frame,
    enums::{Color, Font, Align},
};

pub fn display_load_avg_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 200)
        .with_label("Load Average Monitor");
    wind.set_color(Color::from_rgb(245, 245, 245));
    // Create frame for displaying load averages
    let mut load_frame = Frame::default()
        .with_size(380, 180)
        .with_pos(10, 10);
    load_frame.set_label_size(20);
    load_frame.set_label_color(Color::from_rgb(51, 122, 183));
    load_frame.set_align(Align::Center);

    wind.end();
    wind.show();

    let sys = StatSystem::new();
    let (sender, receiver) = app::channel::<()>();

    // Spawn update thread
    std::thread::spawn(move || {
        while RUNNING.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_secs(2));
            sender.send(());
        }
    });

    while app.wait() {
        if let Some(_) = receiver.recv() {
            if let Ok(loadavg) = sys.load_average() {
                // Color coding based on load values
                let color = if loadavg.one > 2.0 {
                    Color::from_rgb(217, 83, 79)  // Red for high load
                } else if loadavg.one > 1.0 {
                    Color::from_rgb(240, 173, 78)  // Orange for medium load
                } else {
                    Color::from_rgb(92, 184, 92)  // Green for low load
                };

                load_frame.set_label_color(color);
                load_frame.set_label(&format!(
                    "System Load Average\n\n\
                     1 minute:  {:.2}\n\
                     5 minutes: {:.2}\n\
                     15 minutes: {:.2}",
                    loadavg.one, loadavg.five, loadavg.fifteen
                ));
            }
            app::redraw();
        }
    }
    Ok(())
}

pub fn display_load_avg_cli() -> Result<(), Box<dyn std::error::Error>> {
    let sys = StatSystem::new();
    let update_interval = Duration::from_secs(5);

    while RUNNING.load(Ordering::SeqCst) {
        match sys.load_average() {
            Ok(loadavg) => {
                print!("\rload average: {:.2}, {:.2}, {:.2}", 
                    loadavg.one, loadavg.five, loadavg.fifteen);
                std::io::stdout().flush().unwrap();
            }
            Err(e) => eprintln!("\rFailed to get load average: {}", e),
        }

        std::thread::sleep(update_interval);
    }
    Ok(())
}


use fltk::{
    app,
    button::Button,
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
    group::Pack,
};
use crate::ctrlc_handler::RUNNING;
use crate::common::Ordering;
use crate::filtering::filter_processes;
use crate::display_filtered::display_filtered_processes;

pub fn display_filter_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .with_label("Process Filter")
        .center_screen();

    let mut pack = Pack::new(10, 10, 380, 280, "");
    pack.set_spacing(10);

    let mut title = Frame::default()
        .with_size(380, 40)
        .with_label("Filter Processes");
    title.set_label_size(16);

    let mut pid_input = Input::default()
        .with_size(380, 30)
        .with_label("PID (optional):");

    let mut name_input = Input::default()
        .with_size(380, 30)
        .with_label("Process Name (optional):");

    let mut cpu_input = Input::default()
        .with_size(380, 30)
        .with_label("Min CPU Usage % (optional):");

    let mut filter_btn = Button::default()
        .with_size(380, 40)
        .with_label("Apply Filter");
    filter_btn.set_color(fltk::enums::Color::from_rgb(91, 192, 222));
    filter_btn.set_label_color(fltk::enums::Color::White);

    pack.end();
    wind.end();
    wind.show();

    filter_btn.set_callback(move |_| {
        let pid = pid_input.value().parse::<i32>().ok();
        let name = if name_input.value().is_empty() { None } else { Some(name_input.value()) };
        let cpu = cpu_input.value().parse::<f32>().ok();

        // Create a new window for results
        let mut results_wind = Window::default()
            .with_size(800, 600)
            .with_label("Filtered Results")
            .center_screen();
        
        results_wind.show();
        
        // Start the display in a separate thread to keep UI responsive
        std::thread::spawn(move || {
            let _ = display_filtered_processes(pid, name.as_deref(), cpu);
        });
    });

    while app.wait() {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}
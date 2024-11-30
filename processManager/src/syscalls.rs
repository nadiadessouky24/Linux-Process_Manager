use std::process::Command;
use std::io::{self, Write};
use fltk::{
    app,
    prelude::*,
    window::Window,
    frame::Frame,
    input::Input,
    button::Button,
    text::TextDisplay,
    text::TextBuffer,
    enums::{Color, FrameType},
};
use crate::common::RUNNING;
use std::sync::atomic::Ordering;
use crate::input;

pub fn syscalls_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(600, 400)
        .with_label("System Calls");
    wind.set_color(Color::from_rgb(245, 245, 245));

    // Create header with explanation
    let mut header = Frame::default()
        .with_size(580, 50)  // Made taller for two lines
        .with_pos(10, 10)
        .with_label("System Call Interface\nEnter a command (like 'ls', 'ps', 'pwd') and its arguments");
    header.set_label_color(Color::from_rgb(51, 122, 183));
    header.set_label_size(16);

    // Create command input with better label
    let mut command_input = Input::default()
        .with_size(400, 25)
        .with_pos(90, 70)  // Adjusted position
        .with_label("Command:");
    command_input.set_color(Color::White);
    command_input.set_tooltip("Enter system commands like 'ls', 'ps', 'pwd'");

    // Create arguments input with better label and example
    let mut args_input = Input::default()
        .with_size(400, 25)
        .with_pos(90, 105)  // Adjusted position
        .with_label("Arguments:");
    args_input.set_color(Color::White);
    args_input.set_tooltip("Enter command arguments (e.g., '-l' for 'ls -l')");

    // Create execute button
    let mut execute_btn = Button::default()
        .with_size(100, 25)
        .with_pos(500, 105)
        .with_label("Execute");
    execute_btn.set_color(Color::from_rgb(51, 122, 183));
    execute_btn.set_label_color(Color::White);

    // Add example frame
    let mut example = Frame::default()
        .with_size(580, 30)
        .with_pos(10, 140)
        .with_label("Examples: 'ls -l', 'ps aux', 'pwd', 'whoami'");
    example.set_label_color(Color::from_rgb(119, 119, 119));
    example.set_label_size(12);

    // Create output display with label
    let mut output_frame = Frame::default()
        .with_size(580, 20)
        .with_pos(10, 170)
        .with_label("Command Output:");
    output_frame.set_label_color(Color::from_rgb(51, 122, 183));

    let mut output_display = TextDisplay::default()
        .with_size(580, 190)  // Adjusted size
        .with_pos(10, 200);   // Adjusted position
    output_display.set_frame(FrameType::BorderBox);
    output_display.set_color(Color::White);
    
    let mut buffer = TextBuffer::default();
    buffer.set_text("Output will appear here after executing a command.");
    output_display.set_buffer(buffer.clone());

    wind.end();
    wind.show();

    execute_btn.set_callback({
        let mut buffer = buffer.clone();
        let command_input = command_input.clone();
        let args_input = args_input.clone();
        
        move |_| {
            let command = command_input.value();
            let args_str = args_input.value();
            let args: Vec<&str> = args_str.split_whitespace().collect();

            if command.is_empty() {
                buffer.set_text("Please enter a command.");
                return;
            }

            // Special handling for proctree command
            if command.to_lowercase() == "proctree" {
                match Command::new("pstree")
                    .arg("-p")
                    .output() {
                        Ok(output) => {
                            if output.status.success() {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                buffer.set_text(&stdout);
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                buffer.set_text(&format!("Error: {}", stderr));
                            }
                        }
                        Err(e) => buffer.set_text(&format!("Failed to execute command: {}", e)),
                }
                return;
            }

            // Execute the command
            match Command::new(&command)
                .args(&args)
                .output() {
                    Ok(output) => {
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            buffer.set_text(&stdout);
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            buffer.set_text(&format!("Error: {}", stderr));
                        }
                    }
                    Err(e) => buffer.set_text(&format!("Failed to execute command: {}", e)),
            }
        }
    });

    while app.wait() {
        if !RUNNING.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}

pub fn syscalls_cli() {
    loop {
        let user_command = input::get_user_input("Enter a command (or 'exit' to quit): ");
        
        if user_command == "exit" {
            break;
        }
        
        // Check for process tree command
        if user_command.to_lowercase() == "proctree" {
            let output = Command::new("pstree")
                .arg("-p")
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                io::stdout().flush().expect("Error flushing stdout");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: {}", stderr);
            }
        } else {
            // Handle other commands
            let args_input = input::get_user_input("Enter Arguments: ");
            let args: Vec<&str> = args_input.split_whitespace().collect();

            let output = Command::new(&user_command)
                .args(&args)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                io::stdout().flush().expect("Error flushing stdout");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: {}", stderr);
            }
        }
    }
}


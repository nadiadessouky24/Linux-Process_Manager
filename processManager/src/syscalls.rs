use std::process::{Command, Stdio};
use std::io::{self, Write};
use crate::input;
use input::get_user_input;

pub fn syscalls() {
    loop {
        let user_command = input::get_user_input("Enter a command (or 'exit' to quit): ");
        
        if user_command == "exit" {
            break;
        }
        
        // Check for process tree command
        if user_command.to_lowercase() == "proctree" {
            let output = Command::new("pstree")
                .arg("-p") // Show PIDs
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


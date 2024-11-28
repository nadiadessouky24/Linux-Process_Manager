use fltk::{
    app,
    prelude::*,
    window::Window,
    tree::Tree,
    frame::Frame,
    text::{TextDisplay, TextBuffer},
    enums::{Color, FrameType},
    group::Flex,
};
use std::process::Command;
use std::collections::HashMap;
use crate::ctrlc_handler::RUNNING;
use std::sync::atomic::Ordering;

#[derive(Clone)]
struct ProcessInfo {
    pid: String,
    name: String,
    ppid: String,
    user: String,
    cpu: String,
    memory: String,
    state: String,
    command: String,
}

pub fn display_process_tree_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(1200, 600)
        .with_label("Process Tree Viewer");
    wind.set_color(Color::from_rgb(245, 245, 245));

    let mut flex = Flex::default()
        .with_size(1180, 580)
        .with_pos(10, 10)
        .row();

    // Left side - Tree view
    let mut left_group = Flex::default().column();
    
    let mut tree_header = Frame::default()
        .with_label("Process Tree (Click + or - to expand/collapse)");
    tree_header.set_label_color(Color::from_rgb(51, 122, 183));
    tree_header.set_label_size(16);

    let mut tree = Tree::default();
    tree.set_selection_color(Color::from_rgb(51, 122, 183));
    tree.set_show_root(false);
    tree.set_show_collapse(true);  // Show +/- symbols
    tree.set_connector_color(Color::from_rgb(128, 128, 128));
    tree.set_line_spacing(4);  // Add some spacing between items

    left_group.set_size(&tree_header, 30);
    left_group.end();

    // Right side - Process details
    let mut right_group = Flex::default().column();
    
    let mut details_header = Frame::default()
        .with_label("Process Details");
    details_header.set_label_color(Color::from_rgb(51, 122, 183));
    details_header.set_label_size(16);

    let mut details_display = TextDisplay::default();
    details_display.set_frame(FrameType::BorderBox);
    details_display.set_color(Color::White);
    
    right_group.set_size(&details_header, 30);
    right_group.end();

    flex.end();
    wind.end();
    wind.show();

    let mut buffer = TextBuffer::default();
    details_display.set_buffer(buffer.clone());

    // Get process information
    let processes = get_all_processes()?;
    let process_tree = build_process_tree(&processes);

    // Populate initial tree with root processes (processes with PPID 0 or 1)
    for (pid, process) in &processes {
        if process.ppid == "0" || process.ppid == "1" {
            let label = format!("{} ({})", process.name, process.pid);
            if let Some(mut item) = tree.add(&label) {
                if process_tree.contains_key(pid) {
                    // Add a hidden dummy child to make the node expandable
                    let dummy_path = format!("{}/.", item.label().unwrap_or_default());
                    tree.add(&dummy_path);
                    item.close();  // Start collapsed
                }
            }
        }
    }

    // Handle tree events
    tree.set_callback({
        let mut buffer = buffer.clone();
        let processes = processes.clone();
        let process_tree = process_tree.clone();
        
        move |t| {
            if let Some(item) = t.first_selected_item() {
                // Extract PID from the label
                if let Some(label) = item.label() {
                    if let Some(pid) = label.rfind('(').and_then(|i| {
                        label[i+1..].trim_end_matches(')').to_string().into()
                    }) {
                        // Update process details
                        if let Some(process) = processes.get(&pid) {
                            let details = format!(
                                "Process Details:\n\n\
                                 PID: {}\n\
                                 Name: {}\n\
                                 Parent PID: {}\n\
                                 User: {}\n\
                                 CPU Usage: {}%\n\
                                 Memory Usage: {}%\n\
                                 State: {}\n\
                                 Command: {}\n",
                                process.pid,
                                process.name,
                                process.ppid,
                                process.user,
                                process.cpu,
                                process.memory,
                                process.state,
                                process.command
                            );
                            buffer.set_text(&details);
                        }

                        // Handle expansion
                        if item.is_open() {
                            let item_path = item.label().unwrap_or_default();
                            
                            t.begin();
                            // Remove hidden dummy node
                            if let Some(dummy_path) = t.find_item(&format!("{}/.", item_path)) {
                                t.remove(&dummy_path);
                            }
                            
                            // Add actual children
                            if let Some(children) = process_tree.get(&pid) {
                                for child_pid in children {
                                    if let Some(child_process) = processes.get(child_pid) {
                                        let child_label = format!("{} ({})", 
                                            child_process.name, child_process.pid);
                                        // Add child under the current item's path
                                        let child_path = format!("{}/{}", item_path, child_label);
                                        if let Some(mut child_item) = t.add(&child_path) {
                                            if process_tree.contains_key(child_pid) {
                                                // Add hidden dummy node to make it expandable
                                                let dummy_path = format!("{}/.", child_path);
                                                t.add(&dummy_path);
                                                child_item.close();  // Start collapsed
                                            }
                                        }
                                    }
                                }
                            }
                            t.end();
                        }
                    }
                }
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

fn get_all_processes() -> Result<HashMap<String, ProcessInfo>, Box<dyn std::error::Error>> {
    let output = Command::new("ps")
        .args(&["ax", "-o", "pid,ppid,user,%cpu,%mem,state,comm,command", "--no-headers"])
        .output()?;

    let mut processes = HashMap::new();
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    for line in output_str.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 8 {
            let pid = fields[0].to_string();
            processes.insert(pid.clone(), ProcessInfo {
                pid,
                ppid: fields[1].to_string(),
                user: fields[2].to_string(),
                cpu: fields[3].to_string(),
                memory: fields[4].to_string(),
                state: fields[5].to_string(),
                name: fields[6].to_string(),
                command: fields[7..].join(" "),
            });
        }
    }
    
    Ok(processes)
}

fn build_process_tree(processes: &HashMap<String, ProcessInfo>) -> HashMap<String, Vec<String>> {
    let mut tree = HashMap::new();
    
    for (pid, process) in processes {
        tree.entry(process.ppid.clone())
            .or_insert_with(Vec::new)
            .push(pid.clone());
    }
    
    tree
}

fn populate_tree(
    tree: &mut Tree,
    process_tree: &HashMap<String, Vec<String>>,
    processes: &HashMap<String, ProcessInfo>,
    current_pid: &str,
) {
    if let Some(children) = process_tree.get(current_pid) {
        for child_pid in children {
            if let Some(process) = processes.get(child_pid) {
                let label = format!("{} ({})", process.name, process.pid);
                tree.add(&label);
                
                // Recursively add children
                populate_tree(tree, process_tree, processes, child_pid);
            }
        }
    }
}
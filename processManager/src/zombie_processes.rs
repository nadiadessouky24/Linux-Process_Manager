use sysinfo::{System, SystemExt, ProcessExt};
use std::io::Write;
use fltk::{
    app,
    prelude::*,
    window::Window,
    frame::Frame,
    table::*,
    draw,
    enums::{Color, Font, Align, FrameType},
};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use crate::ctrlc_handler::RUNNING;
use std::sync::atomic::Ordering;

struct ZombieProcess {
    pid: i32,
    name: String,
}

pub fn display_zombie_processes_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .with_label("Zombie Processes Monitor");
    wind.set_color(Color::from_rgb(245, 245, 245));

    // Create header
    let mut header = Frame::default()
        .with_size(380, 40)
        .with_pos(10, 10)
        .with_label("Zombie Processes");
    header.set_label_color(Color::from_rgb(51, 122, 183));
    header.set_label_size(20);

    // Create table for zombie processes
    let mut table = Table::default()
        .with_size(380, 240)
        .with_pos(10, 50);
    table.set_rows(1);
    table.set_cols(2);
    table.set_col_header(true);
    table.set_col_width_all(190);
    table.set_row_height_all(25);
    table.set_selection_color(Color::from_rgb(200, 215, 255));

    let zombies = Rc::new(RefCell::new(Vec::<ZombieProcess>::new()));
    let zombies_clone = zombies.clone();

    table.draw_cell(move |t, mut ctx, row, col, x, y, w, h| {
        match ctx {
            TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
            TableContext::ColHeader => {
                let headers = ["PID", "Process Name"];
                if let Some(header) = headers.get(col as usize) {
                    draw_header(t, &mut ctx, x, y, w, h, header);
                }
            },
            TableContext::Cell => {
                let zoms = zombies_clone.borrow();
                if let Some(zombie) = zoms.get((row as usize) - 1) {
                    let value = match col {
                        0 => zombie.pid.to_string(),
                        1 => zombie.name.clone(),
                        _ => String::new(),
                    };
                    draw_cell(t, &mut ctx, x, y, w, h, &value);
                }
            },
            _ => (),
        }
    });

    wind.end();
    wind.show();

    let (sender, receiver) = app::channel::<()>();

    std::thread::spawn(move || {
        while RUNNING.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_secs(2));
            sender.send(());
        }
    });

    while app.wait() {
        if let Some(_) = receiver.recv() {
            let mut system = System::new_all();
            system.refresh_all();
            
            let mut zombie_list = Vec::new();
            
            for (pid, process) in system.processes() {
                if let sysinfo::ProcessStatus::Zombie = process.status() {
                    zombie_list.push(ZombieProcess {
                        pid: *pid,
                        name: process.name().to_string(),
                    });
                }
            }

            // Update table
            *zombies.borrow_mut() = zombie_list;
            let num_zombies = zombies.borrow().len();
            
            // Update header text
            if num_zombies == 0 {
                header.set_label("No zombie processes found");
            } else {
                header.set_label(&format!("Zombie Processes Found: {}", num_zombies));
            }
            
            // Update table rows
            table.set_rows((num_zombies + 1) as i32); // +1 for header
            table.redraw();
            app::redraw();
        }
    }
    Ok(())
}

pub fn display_zombie_processes_cli() {
    let mut system = System::new_all();
    system.refresh_all();

    let mut zombie_found = false;
    
    for (_, process) in system.processes() {
        if let sysinfo::ProcessStatus::Zombie = process.status() {
            println!("Zombie Process: PID {}", process.pid());
            zombie_found = true;
        }
    }
    
    if !zombie_found {
        println!("No zombie processes found.");
    }
}

fn draw_header(_table: &mut Table, _ctx: &mut TableContext, x: i32, y: i32, w: i32, h: i32, text: &str) {
    draw::push_clip(x, y, w, h);
    draw::set_draw_color(Color::from_rgb(51, 122, 183));
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::from_rgb(51, 122, 183));
    draw::set_draw_color(Color::White);
    draw::draw_text2(text, x, y, w, h, Align::Center);
    draw::pop_clip();
}

fn draw_cell(_table: &mut Table, _ctx: &mut TableContext, x: i32, y: i32, w: i32, h: i32, text: &str) {
    draw::push_clip(x, y, w, h);
    
    let row_color = if (y / h) % 2 == 0 {
        Color::from_rgb(240, 240, 255)
    } else {
        Color::White
    };
    
    draw::set_draw_color(row_color);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, row_color);
    draw::set_draw_color(Color::from_rgb(51, 122, 183));  // Blue text
    draw::draw_text2(text, x, y, w, h, Align::Center);
    draw::pop_clip();
}



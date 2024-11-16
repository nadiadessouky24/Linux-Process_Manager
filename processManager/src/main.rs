mod load_avg; 
mod process_display;
mod input;
mod syscalls;
mod common;
mod ctrlc_handler;
mod display_zombie; 
use ctrlc_handler::{exiting_loop,RUNNING};
use syscalls::syscalls;
use crate::common::{Arc,ctrlc,Ordering};
use input::get_user_input; 
use process_display::display_process_info; 
use load_avg::display_load_avg; 
use display_zombie::display_zombie_processes; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    exiting_loop();
    loop {
            let input = input::get_user_input("\n To Display Process information enter 'diplay', \n To display load average enter 'loadavg' \n To run system calls enter 'command' \n To exit enter 'exit': ");

            match input.as_str() {
                "display" => 
                {
                    RUNNING.store(true, Ordering::SeqCst); 
                    display_process_info()?;
                }
                "loadavg" => 
                {
                    RUNNING.store(true, Ordering::SeqCst); 
                    display_load_avg()?; 
                }
                "command" =>
                {
                   syscalls();
                }
                "zombies" => {
                RUNNING.store(true, Ordering::SeqCst);
                display_zombie_processes()?;  
                }
                "exit" => 
                {
                    break;
                }
                _ => 
                {
                    println!("Unknown command, try again.");
                }

            }
    }
    Ok(())
}

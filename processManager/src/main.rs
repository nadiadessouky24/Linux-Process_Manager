mod process_display;
mod input;
mod syscalls;
mod common;
use crate::common::{Arc,ctrlc,Ordering};
use process_display::display_process_info; 



fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
            let mut input = input::get_user_input("\n GUI/CLI: ");
            input = input.to_uppercase();

            match input.as_str() {
                "CLI" => 
                {
                    display_process_info()?;
                }
                "GUI" => 
                {
                    println!("you chose GUI");
                }
                _ => 
                {
                    println!("Unknown command, try again.");
                }

            }
    }
}
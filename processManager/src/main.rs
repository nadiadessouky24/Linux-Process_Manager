mod load_avg; 
mod process_display; 
use std::io::{self, Write};
use process_display::display_process_info; 
use load_avg::display_load_avg; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Prompt the user
        print!("Enter a command (type 'display' to show process info, 'loadavg' to show load average, 'exit' to quit): ");
        io::stdout().flush()?; // Ensure prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "display" => 
            {
                display_process_info()?;
            }
            "loadavg" => 
            {
                display_load_avg()?; 
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

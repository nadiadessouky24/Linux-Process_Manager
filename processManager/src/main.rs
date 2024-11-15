mod load_avg; 
mod process_display;
mod input;
mod syscalls;
use syscalls::syscalls;
use input::get_user_input; 
use process_display::display_process_info; 
use load_avg::display_load_avg; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {

            let input = input::get_user_input("To Display Process information enter diplay, \n To display load average enter loadavg \n To run system calls enter command \n To exit enter exit: ");

            match input.as_str() {
                "display" => 
                {
                    display_process_info()?;
                }
                "loadavg" => 
                {
                    display_load_avg()?; 
                }
                "command" =>
                {
                   syscalls();
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

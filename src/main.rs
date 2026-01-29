use std::env;
use std::error::Error;
use task_cli::commands;
use task_cli::models::list::List;

fn main() -> Result<(), Box<dyn Error>> {
    // Ensure storage exists
    List::init()?;

    // Parse CLI arguments
    let mut args = env::args().skip(1);
    let command = args.next();
    let rest: Vec<String> = args.collect();

    // Dispatch command
    match command.as_deref() {
        Some("add") => {
            println!("{}", commands::add::run(&rest)?);
        }
        Some("update") => {
            println!("{}", commands::update::run(&rest)?);
        }
        Some("delete") => {
            println!("{}", commands::delete::run(&rest)?);
        }
        Some("mark-in-progress") => {
            println!("{}", commands::mark::in_progress(&rest)?);
        }
        Some("mark-done") => {
            println!("{}", commands::mark::done(&rest)?);
        }
        Some("list") => {
            commands::list::run(&rest)?;
        }
        _ => {
            eprintln!("Invalid command");
        }
    }

    Ok(())
}

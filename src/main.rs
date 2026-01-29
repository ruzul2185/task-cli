// Import environment handling for CLI arguments.
use std::env;
// Import standard error handling.
use std::error::Error;
// Import your commands module (add, update, delete, mark, list).
use task_cli::commands;
// Import the List model to handle storage initialization.
use task_cli::models::list::List;

fn main() -> Result<(), Box<dyn Error>> {
    // Ensure the JSON storage file exists. If not, initialize it.
    List::init()?;

    // Parse CLI arguments, skipping the first argument (program name).
    let mut args = env::args().skip(1);
    // The first argument after the program name is the command.
    let command = args.next();
    // The rest of the arguments are passed to the command handler.
    let rest: Vec<String> = args.collect();

    // Dispatch the command based on the first argument.
    match command.as_deref() {
        Some("add") => {
            // Call the `add` command and print its result.
            println!("{}", commands::add::run(&rest)?);
        }
        Some("update") => {
            // Call the `update` command and print its result.
            println!("{}", commands::update::run(&rest)?);
        }
        Some("delete") => {
            // Call the `delete` command and print its result.
            println!("{}", commands::delete::run(&rest)?);
        }
        Some("mark-in-progress") => {
            // Call the `mark::in_progress` command and print its result.
            println!("{}", commands::mark::in_progress(&rest)?);
        }
        Some("mark-done") => {
            // Call the `mark::done` command and print its result.
            println!("{}", commands::mark::done(&rest)?);
        }
        Some("list") => {
            // Call the `list` command, which prints tasks directly.
            commands::list::run(&rest)?;
        }
        _ => {
            // Handle invalid commands.
            eprintln!("Invalid command");
        }
    }

    // Return success.
    Ok(())
}

// Import the List and the Task structs from the models module.
use crate::models::list::List;
use crate::models::task::Task;

// Import standard error handling.
use std::error::Error;

/// Adds a new task to the task list.
/// 
/// Adds a new task to the task list.
/// 
/// # Arguments
/// * `args` - A slice of strings where the first element is expected to be the task description.
/// 
/// # Returns
/// * `Ok(String)` with a success message if the task is added successfully.
/// * `Err(Box<dyn Error>)` if there is an error (wrong arguments or IO/serialization error).
pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    // Ensure exctly one argument (the task description) is provided.
    if args.len() != 1 {
        // Returns an error with a usage message if the user provided wrong arguments.
        return Err("Usage: add <description>".into());
    }

    // Load the existing task list from the JSON file.
    let mut list = List::load()?;

    // Get the next available task ID from the list.
    let id = list.next_id;

    // Create a new Task witht the given ID and description.
    // Push the new task oto the list.
    list.list.push(Task::new(id, args[0].clone()));

    // Increment the next_id for future tasks.
    list.next_id += 1;

    // Save the updated task list back to the JSON file.
    list.save()?;

    // REturn a success message including the new task's ID.
    Ok(format!("Task added successfully (ID: {id})"))
}

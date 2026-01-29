// Import standard error handling.
use std::error::Error;

// Import Utc from chrono for timestamp updates.
use chrono::Utc;

// Import the List struct from you models module.
use crate::models::list::List;

/// Updates the description of an existing task.
///
/// # Arguments
/// * `args` - A slice of strings where:
///     - `args[0]` is expected to be the task ID
///     - `args[1..]` is the new description (can be multiple words)
///
/// # Returns
/// * `Ok(String)` on success, with a message including the task ID.
/// * `Err(Box<dyn Error>)` if the ID is missing, invalid, or the task does not exist.
pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    // Ensure at least 2 arguments: the task ID and new description.
    if args.len() < 2 {
        return Err("Usage: update <id> <description>".into());
    }

    // Parse the first argument as a usize (the task ID).
    // Return an error if parsing fails.
    let id: usize = args[0].parse().map_err(|_| "Invalid task id")?;
    let description = args[1].clone();

    // Load the current task list from the JSON file.
    let mut list = List::load()?;

    // Find a mutable reference to the task with the given ID.
    // Return an error if the task is not found.
    let task = list
        .find_mut(id)
        .ok_or("Task not found. It may have been deleted or the id is invalid")?;

    // Update the task's description
    task.description = description;

    // Update the `updated_at` timestamp to the current time.
    task.updated_at = Utc::now();

    // Save the updated task list back to the JSON file.
    list.save()?;

    // Return a success message including the task ID.
    Ok(format!("Task updated successfully (ID: {id})"))
}

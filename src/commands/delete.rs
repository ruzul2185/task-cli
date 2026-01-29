// Import standard error handling.
use std::error::Error;

// Import the List struct from your models module.
use crate::models::list::List;

/// Deletes a task from the task list by its ID.
///
/// # Arguments
/// * `args` - A slice of strings where the first element should be the task ID.
///
/// # Returns
/// * `Ok(String)` with a success message if the task was deleted.
/// * `Err(Box<dyn Error>)` if there is an error (wrong arguments, invalid ID, or task not found).
pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    // Ensure exactly one argument (the task ID) is provided.
    if args.len() != 1 {
        return Err("Usage: delete <id>".into());
    }

    // Parse the first argument as a usize (task ID).
    // If parsing fails, return an "Invalid task id" error.
    let id: usize = args[0].parse().map_err(|_| "Invalid task id")?;

    // Load the existing task lsit from the JSON file.
    let mut list = List::load()?;

    // Attempt to remove the task with the given ID.
    // `remove` returns `true` if a task was removed, `false` if not found.
    if !list.remove(id) {
        return Err("Task not found. It may have been deleted or the id is invalid".into());
    }

    // Save the updated task list back to the JSON file.
    list.save()?;

    // Return a success message including the deleted task's ID.
    Ok(format!("Task deleted successfully (ID: {id})"))
}

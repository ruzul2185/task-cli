// Import Utc from chrono for timestamp handling.
use chrono::Utc;

// Import the List and Status structs from your models.
use crate::models::list::List;
use crate::models::task::Status;

// Import standard error handling.
use std::error::Error;

/// Mark a task as "In Progress".
///
/// # Arguments
/// * `args` - A slice of strings where the first element should be the task ID.
///
/// # Returns
/// * `Ok(String)` if the task status is updated successfully.
/// * `Err(Box<dyn Error>)` if the task ID is missing, invalid, or task not found.
pub fn in_progress(args: &[String]) -> Result<String, Box<dyn Error>> {
    // Delegate to the common `update_status` function.
    update_status(args, Status::InProgress)
}

/// Mark a task as "Done".
///
/// # Arguments
/// * `args` - A slice of strings where the first element should be the task ID.
///
/// # Returns
/// * `Ok(String)` if the task status is updated successfully.
/// * `Err(Box<dyn Error>)` if the task ID is missing, invalid, or task not found.
pub fn done(args: &[String]) -> Result<String, Box<dyn Error>> {
    // Delegate to the common `update_status` function.
    update_status(args, Status::Done)
}

/// Common helper function to update the status of a task.
///
/// # Arguments
/// * `args` - Slice of strings where the first element is expected to be the task ID.
/// * `status` - The new status to assign to the task.
///
/// # Returns
/// * `Ok(String)` on success, including the task ID.
/// * `Err(Box<dyn Error>)` if the ID is missing, cannot be parsed, or task not found.
fn update_status(
    args: &[String],
    status: Status,
) -> Result<String, Box<dyn Error>> {
    // Get the first argument as the task ID, or return as error if missing.
    let id: usize = args.get(0).ok_or("Missing id")?.parse()?;

    // Load the task list from the JSON file.
    let mut list = List::load()?;

    // Find the task by ID; return an error if not found.
    let task = list.find_mut(id).ok_or("Task not found")?;

    // Update the task's status and update the timestamp.
    task.status = status;
    task.updated_at = Utc::now();

    // Save the updated list back to disk.
    list.save()?;

    // Return a success message including the updated task ID.
    Ok(format!("Task updated successfully (ID: {id})"))
}

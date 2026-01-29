// Import standard error handling.
use std::error::Error;

// Import List and Status from your models.
use crate::models::list::List;
use crate::models::task::Status;

/// Lists tasks, optionally filtered by status.
///
/// # Arguments
/// * `args` - A slice of strings where the first element can optionally
///            specify a status filter: "todo", "in-progress", or "done".
///
/// # Returns
/// * `Ok(())` if tasks are listed successfully.
/// * `Err(Box<dyn Error>)` if invalid arguments are provided.
pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    // Ensure at most one argument is provided (the optional status filter).
    if args.len() > 1 {
        return Err("Usage: list [todo|in-progress|done]".into());
    }

    // Load the task list from the JSON file.
    let list = List::load()?;

    // Determine the optional filter (if provided).
    let filter = args.get(0).map(|s| s.as_str());

    // Filter tasks based on the filter arguments.
    // If no filter is provided, return all tasks.
    let tasks: Vec<_> = match filter {
        Some("todo") => list.list.iter().filter(|t| t.status == Status::Todo).collect(),
        Some("in-progress") => list.list.iter().filter(|t| t.status == Status::InProgress).collect(),
        Some("done") => list.list.iter().filter(|t| t.status == Status::Done).collect(),
        // Handle invaid filter
        Some(other) => return Err(format!("Unknown filter: {other}").into()),
        None => list.list.iter().collect(), // No filter, includes all tasks.
    };

    // if the filtered list is empty, inform the user.
    if tasks.is_empty() {
        println!("The list is empty");
        return Ok(());
    }

    // Print each task. `Task` implements Display, so this prints nicely.
    for task in tasks {
        println!("{task}\n");
    }

    Ok(())
}

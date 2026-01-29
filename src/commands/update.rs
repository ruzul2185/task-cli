use std::error::Error;
use chrono::Utc;
use crate::models::list::List;

pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Usage: update <id> <description>".into());
    }

    let id: usize = args[0].parse().map_err(|_| "Invalid task id")?;
    let description = args[1..].join(" ");

    let mut list = List::load()?;
    let task = list
        .find_mut(id)
        .ok_or("Task not found. It may have been deleted or the id is invalid")?;

    task.description = description;
    task.updated_at = Utc::now();

    list.save()?;
    Ok(format!("Task updated successfully (ID: {id})"))
}

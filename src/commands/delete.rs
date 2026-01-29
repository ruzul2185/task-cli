use std::error::Error;
use crate::models::list::List;

pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Usage: delete <id>".into());
    }

    let id: usize = args[0].parse().map_err(|_| "Invalid task id")?;
    let mut list = List::load()?;

    if !list.remove(id) {
        return Err("Task not found. It may have been deleted or the id is invalid".into());
    }

    list.save()?;
    Ok(format!("Task deleted successfully (ID: {id})"))
}

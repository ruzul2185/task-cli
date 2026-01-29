use crate::models::list::List;
use crate::models::task::Task;
use std::error::Error;

pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Usage: add <description>".into());
    }

    let mut list = List::load()?;
    let id = list.next_id;

    list.list.push(Task::new(id, args[0].clone()));
    list.next_id += 1;

    list.save()?;
    Ok(format!("Task added successfully (ID: {id})"))
}

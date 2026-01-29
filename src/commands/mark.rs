use chrono::Utc;
use crate::models::list::List;
use crate::models::task::Status;
use std::error::Error;

pub fn in_progress(args: &[String]) -> Result<String, Box<dyn Error>> {
  update_status(args, Status::InProgress)
}

pub fn done(args: &[String]) -> Result<String, Box<dyn Error>> {
  update_status(args, Status::Done)
}

fn update_status(
  args: &[String],
  status: Status,
) -> Result<String, Box<dyn Error>> {
  let id: usize = args.get(0).ok_or("Missing id")?.parse()?;
  let mut list = List::load()?;

  let task = list.find_mut(id).ok_or("Task not found")?;
  task.status = status;
  task.updated_at = Utc::now();

  list.save()?;
  Ok(format!("Task updated successfully (ID: {id})"))
}

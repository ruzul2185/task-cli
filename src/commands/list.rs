use std::error::Error;
use crate::models::list::List;
use crate::models::task::Status;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() > 1 {
        return Err("Usage: list [todo|in-progress|done]".into());
    }

    let list = List::load()?;

    let filter = args.get(0).map(|s| s.as_str());
    let tasks: Vec<_> = match filter {
        Some("todo") => list.list.iter().filter(|t| t.status == Status::Todo).collect(),
        Some("in-progress") => list.list.iter().filter(|t| t.status == Status::InProgress).collect(),
        Some("done") => list.list.iter().filter(|t| t.status == Status::Done).collect(),
        Some(other) => return Err(format!("Unknown filter: {other}").into()),
        None => list.list.iter().collect(),
    };

    if tasks.is_empty() {
        println!("The list is empty");
        return Ok(());
    }

    for task in tasks {
        println!("{task}\n");
    }

    Ok(())
}

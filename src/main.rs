use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::fmt;
use chrono::{DateTime, Utc};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Status::Todo => "To do",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            description,
            status: Status::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Created: {}", self.created_at)?;
        writeln!(f, "Updated: {}", self.updated_at)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    next_id: usize,
    list: Vec<Task>,
}

impl List {
    fn new() -> Self {
        Self {
            next_id: 1,
            list: Vec::new(),
        }
    }

    fn file_init(input: &str) -> Result<(), Box<dyn std::error::Error>> {
       if !fs::exists(input)? {
        fs::write(input, &serde_json::to_string_pretty(&List::new())?)?;
       } 
       Ok(())
    }

    fn load_file_to_list(input: &str) -> Result<List, Box<dyn std::error::Error>> {
        let list: String = fs::read_to_string(input)?;
        let list_json: List = serde_json::from_str(&list)?;
        Ok(list_json)
    }

    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

static LIST_FILE: &str = "list.json";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut arguments = env::args().skip(1);
    List::file_init(LIST_FILE)?;
    let cmd = arguments.next();
    let cmd_args: Vec<String> = arguments.collect();
    match cmd.as_deref() {
        Some("add") => {
            //todo!();
            let msg = add_task(&cmd_args)?;
            println!("{msg}");
        }
        Some("update") => {
            let msg = update_task(&cmd_args)?;
            println!("{msg}");
        }
        Some("delete") => {
            let msg = delete_task(&cmd_args)?;
            println!("{msg}");
        }
        Some("mark-in-progress") => {
            let msg = mark_task_as_in_progress(&cmd_args)?;
            println!("{msg}");
        }
        Some("mark-done") => {
            let msg = mark_task_as_done(&cmd_args)?;
            println!("{msg}");
        }
        Some("list") => {
            show_list(&cmd_args)?;
        }
        _ => {
            eprintln!("Invalid command");
        }
    }
    Ok(())
}

fn show_list(arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if arguments.len() > 1 {
        return Err("Usage: list done/todo/in-progress".into());
    }

    let list = List::load_file_to_list(LIST_FILE)?;

    let filter = arguments.get(0).map(|s| s.as_str());
    let tasks_to_show: Vec<_> = match filter {
        Some("todo") => list.list.iter().filter(|t| t.status == Status::Todo).collect(),
        Some("in-progress") => list.list.iter().filter(|t| t.status == Status::InProgress).collect(),
        Some("done") => list.list.iter().filter(|t| t.status == Status::Done).collect(),
        Some(other) => return Err(format!("Unknown argument: {}", other).into()),
        None => list.list.iter().collect(),
    };

    if tasks_to_show.is_empty() {
        println!("The list is empty");
        return Ok(());
    }

    for task in tasks_to_show {
        println!("{task}\n");
    }

    Ok(())
}

fn mark_task_as_done(arguments: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    if arguments.len() != 1 {
        return Err("Usage: mark-done <id>".into());
    }

    let uuid: usize = arguments[0].parse().map_err(|_| "Invalid task id")?;

    let mut list = List::load_file_to_list(LIST_FILE)?;

    let task = list
        .list
        .iter_mut()
        .find(|task| task.id == uuid)
        .ok_or("Task not found. It may have been deleted or the id is invalid")?;

    task.status = Status::Done;
    task.updated_at = Utc::now();

    list.save(LIST_FILE)?;
    // fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    Ok(format!("Task updated successfully (ID: {})", uuid))
}

fn mark_task_as_in_progress(arguments: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    if arguments.len() != 1 {
        return Err("Usage: mark-in-progress <id>".into());
    }

    let uuid: usize = arguments[0].parse().map_err(|_| "Invalid task id")?;

    let mut list = List::load_file_to_list(LIST_FILE)?;

    let task = list
        .list
        .iter_mut()
        .find(|task| task.id == uuid)
        .ok_or("Task not found. It may have been deleted or the id is invalid")?;

    task.status = Status::InProgress;
    task.updated_at = Utc::now();

    list.save(LIST_FILE)?;
    // fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    Ok(format!("Task updated successfully (ID: {})", uuid))
}

fn delete_task(arguments: &[String]) -> Result<String, Box<dyn std::error::Error>>{
    if arguments.len() != 1 {
        return Err("Usage: delete <id>".into());
    }

    let uuid: usize = arguments[0].parse().map_err(|_| "Invalid task id")?;

    let mut list = List::load_file_to_list(LIST_FILE)?;

    let original_len = list.list.len();

    list.list.retain(|task| task.id != uuid);

    if list.list.len() == original_len {
        return Err("Task not found. It may have been deleted or the id is invalid".into());
    }

    list.save(LIST_FILE)?;
    // fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    Ok(format!("Task deleted successfully (ID: {})", uuid))
}

fn update_task(arguments: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    if arguments.len() < 2 {
        return Err("Usage: update <id> <description>".into());
    } 

    let uuid: usize = arguments[0].parse().map_err(|_| "Invalid task id")?;
    let desc: String = arguments[1].trim().to_string();

    let mut list = List::load_file_to_list(LIST_FILE)?;

    let task = list
        .list
        .iter_mut()
        .find(|task| task.id == uuid)
        .ok_or("Task not found. It may have been deleted or the id is invalid")?;

    task.description = desc;
    task.updated_at = Utc::now();

    list.save(LIST_FILE)?;
    // fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    Ok(format!("Task updated successfully (ID: {})", uuid))
}

fn add_task(arguments: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    if arguments.len() != 1 {
        return Err("Usage: add <id> <description>".into());
    }

    let mut list = List::load_file_to_list(LIST_FILE)?;
    let uuid = list.next_id;
    let new_task:Task = Task::new(uuid, arguments[0].to_owned());
    list.list.push(new_task);
    list.next_id += 1;
    list.save(LIST_FILE)?;
    // fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    Ok(format!("Task added successfully (ID: {})", uuid))
}

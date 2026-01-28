use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use chrono::{DateTime, Utc};

#[derive(Default, Serialize, Deserialize, Debug)]
enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
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
            add_task(&cmd_args)?;
        }
        Some("update") => {
            todo!();
        }
        Some("delete") => {
            todo!();
        }
        Some("mark-in-progress") => {
            todo!();
        }
        Some("mark-done") => {
            todo!();
        }
        Some("list") => {
            todo!();
        }
        _ => {
            eprintln!("Invalid command");
        }
    }
    Ok(())
}

fn add_task(arguments: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if arguments.len() != 1 {
        return Err("Too many or few arguments: only 1 argument allowed".into());
    }

    let mut list = List::load_file_to_list(LIST_FILE)?;
    let uuid = list.next_id;
    let new_task:Task = Task::new(uuid, arguments[0].clone());
    list.list.push(new_task);
    list.next_id += 1;
    fs::write(LIST_FILE, serde_json::to_string_pretty(&list)?)?;
    println!("Task added successfully (ID: {uuid})"); 
    Ok(())
}

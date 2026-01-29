// Import the `Task` struct from the `module::task` module in the crate.
use crate::models::task::Task;

// Import serialization and deserialization traits from `serde`.
use serde::{Deserialize, Serialize};

// Import filesystem operations and error handling utilities from std.
use std::fs;
use std::error::Error;

// Define a constant for the JSON file that will store the task list.
const LIST_FILE: &str = "list.json";

// Define a struct representing a task list.
// This struct will be serialized to and deserialized from JSON.
#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    // Tracks the next ID to assign to a new task.
    pub next_id: usize,
    // The vector that holds all Task objects.
    pub list: Vec<Task>,
}

impl List {
    /// Creates a new, empty task list with `next_id` initialized to 1.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            list: Vec::new(),
        }
    }

    /// Initializes he JSON file on disk if it dosen't exist.
    /// Returns a Result to propagate any filesystem or serialization errors.
    pub fn init() -> Result<(), Box<dyn Error>> {
        // Check if the list file exists.
        if !fs::exists(LIST_FILE)? {
            // If the file does not exist, create a new List and write it as pretty JSON.
            fs::write(LIST_FILE, serde_json::to_string_pretty(&Self::new())?)?;
        }
        Ok(())
    }

    /// Loads the task list from the JSON file on disk.
    /// Returns the deserialized `List` object.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // Read the file contents as a string.
        let content = fs::read_to_string(LIST_FILE)?;
        // Deserialize the JSON string into a List struct.
        Ok(serde_json::from_str(&content)?)
    }

    /// Saves the current state of the task list to the JSON file.
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        // Serialize the List to a pretty JSON string and write it to disk.
        fs::write(LIST_FILE, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Finds a mutable reference to a task by its ID.
    /// Returns `Some(&mut Task)` if found, `None` otherwise.
    pub fn find_mut(&mut self, id: usize) -> Option<&mut Task> {
        // Iterate over tasks and return the first one matching the ID.
        self.list.iter_mut().find(|t| t.id == id)
    }

    /// Remove a task by ID from the list.
    /// Returns a `true` if a task was removed, `false` otherwise.
    pub fn remove(&mut self, id: usize) -> bool {
        let len = self.list.len();          // Store the original length.
        self.list.retain(|t| t.id != id);   // Retain only tasks that do NOT match the ID.
        len != self.list.len()              // If lenght changed, a task was removed.
    }
}

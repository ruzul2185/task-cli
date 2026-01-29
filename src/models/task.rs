// Import `DateTime` and `Utc` from the `chrono` crate for timestamp handling.
use chrono::{DateTime, Utc};

// Import `Serialize` and `Deserialize` traits from `serde` for JSON support.
use serde::{Deserialize, Serialize};

// Import `fmt` for custom formatting with the `Display` trait.
use std::fmt;

/// Represents the status of a task.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    /// The task is pending or not started.
    #[default]    
    Todo,
    /// The task is currently in progress.
    InProgress,
    /// The task is completed.
    Done,
}

// Implement `Display` trait for `Status` to allow user-friendly printing.
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match the enum variant to a human-redable string.
        let s = match self {
            Status::Todo => "To do",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        };
        // Write the string to the formatter.
        write!(f, "{s}")
    }
}

/// Represents a single task in the task management system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// Unique identifier of the task.
    pub id: usize,
    /// Text description of the task.
    pub description: String,
    /// Current status of the task (`Todo`, `InProgress`, `Done`).
    pub status: Status,
    /// Timestamp of when the task was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of when the task was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Creates a new `Task` with the given ID and description.
    /// The status is set to `Todo` by default.
    /// `created_at` and `updated_at` are set to the current UTC time.
    pub fn new(id: usize, description: String) -> Self {
        let now = Utc::now();   // Capture the current UTC time
        Self {
            id,
            description,
            status: Status::default(),  // Uses `Todo` as default
            created_at: now,
            updated_at: now,
        }
    }
}

// Implement `Display` for `Task` to allow pretty-printing.
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    #[default]    
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Todo => "To do",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        };
        write!(f, "{s}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
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

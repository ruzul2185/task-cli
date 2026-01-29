use crate::models::task::{Task};
use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

const LIST_FILE: &str = "list.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub next_id: usize,
    pub list: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            list: Vec::new(),
        }
    }

    pub fn init() -> Result<(), Box<dyn Error>> {
        if !fs::exists(LIST_FILE)? {
            fs::write(LIST_FILE, serde_json::to_string_pretty(&Self::new())?)?;
        }
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(LIST_FILE)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        fs::write(LIST_FILE, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn find_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.list.iter_mut().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: usize) -> bool {
        let len = self.list.len();
        self.list.retain(|t| t.id != id);
        len != self.list.len()
    }
}

use crate::base::TodoItem;
use std::fs;
use std::path::Path;

const CACHE_FOLDER: &str = "todos";
const DEFAULT_TOPIC: &str = "general";

pub fn load_todo_items(file_path: &str) -> Result<Vec<TodoItem>, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(file_path)?;

    // Parse the JSON
    let todo_items: Vec<TodoItem> = serde_json::from_str(&json_content)?;

    Ok(todo_items)
}

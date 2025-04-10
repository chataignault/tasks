use crate::base::TodoItem;
use std::fs;
use std::path::Path;

const CACHE_FOLDER: &str = "todos";
const DEFAULT_TOPIC: &str = "general";
const DEFAULT_HISTORY: &str = "history";

pub fn load_todo_items(file_name: &str) -> Result<Vec<TodoItem>, Box<dyn std::error::Error>> {
    _ = fs::create_dir_all(CACHE_FOLDER);
    let file_path = Path::new(CACHE_FOLDER).join(Path::new(file_name));
    if !file_path.exists() {
        _ = fs::File::create_new(&file_path);
    }
    let json_content = fs::read_to_string(file_path)?;
    // Parse the JSON
    let todo_items: Vec<TodoItem> = serde_json::from_str(&json_content)?;

    Ok(todo_items)
}

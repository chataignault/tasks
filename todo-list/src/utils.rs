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

pub fn load_from_file() -> () {
    let path = Path::new(&CACHE_FOLDER);
    // let entries = match fs::read_dir(path) {
    //     Ok(entries) => entries,
    //     Err(_) => {
    //         println!("Error reading directory");
    //         return TodoList::new(&[], &[]);
    //     }
    // };
    // let mut todo_list_topic = TodoList::new(&[], &[]);
    // for entry in entries {
    //     if let Ok(entry) = entry {
    //         if let Some(file_name) = entry.file_name().to_str() {
    //             if let Some(topic) = file_name.split("_").next() {
    //                 println!("Found topic : {}", topic);
    //                 todo_list_topic.items.insert(
    //                     (*topic).to_string(),
    //                     TodoList::load_from_file((*topic).to_string()),
    //                 );
    //             }
    //         }
    //     }
    // }
    // return todo_list_topic;
}

use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    name: String,
    items: HashMap<usize, Todo>,
    next_id: usize,
}

#[derive(Serialize, Deserialize)]
struct TodoListTopic {
    items: HashMap<String, TodoList>,
    current_topic: String,
}


impl TodoList {
    fn new(name: String) -> Self {
        TodoList {
            name: name,
            items: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, description: String) {
        let todo = Todo {
            description,
            completed: false,
        };
        self.items.insert(self.next_id, todo);
        self.next_id += 1;
        self.save_to_file();
    }

    fn mark_complete(&mut self, id: usize) -> bool {
        if let Some(todo) = self.items.get_mut(&id) {
            todo.completed = true;
            self.save_to_file();
            true
        } else {
            false
        }
    }

    fn display(&self) {
        println!("{}: ", self.name);
        for (id, todo) in &self.items {
            let status = if todo.completed { "[x]" } else { "[ ]" };
            println!("{} {}: {}", id, status, todo.description);
        }
    }

    fn save_to_file(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        fs::write(format!("{}_todo_list.json", self.name), serialized).expect("Unable to write file");
    }

    fn load_from_file(name: String) -> Self {
        match fs::read_to_string(format!("{}_todo_list.json", name)) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| TodoList::new(name)),
            Err(_) => TodoList::new(name),
        }
    }
}


impl TodoListTopic {
    fn new() -> Self {
        TodoListTopic {
            items: HashMap::new(),
            current_topic: "".to_string(),
        }
    }

    fn add(&mut self, description: String) {
        let todo = Todo {
            description,
            completed: false,
        };
        let next_id_topic : usize = self.items.get_mut(&self.current_topic).unwrap().next_id;
        self.items.get_mut(&self.current_topic).unwrap().items.insert(next_id_topic, todo);
        self.save_to_file();
    }

    fn mark_complete(&mut self, id: usize) -> bool {
        if let Some(todo) = self.items.get_mut(&self.current_topic).unwrap().items.get_mut(&id) {
            todo.completed = true;
            self.save_to_file();     
            true
        } else {
            false
        }
    }

    fn display(&mut self) {
        self.items.get_mut(&self.current_topic).unwrap().display();
    }

    fn list_topics(&mut self){
        for topic in self.items.keys() {
            println!("{}", topic)
        }
    }


    fn save_to_file(&mut self) {
        self.items.get_mut(&self.current_topic).unwrap().save_to_file();
    }

    fn load_from_file(topic: String) -> Self {
        let folder : String = "todos".to_string();
        let path = Path::new(&folder);
        // List files in folder and get their prefix in front of '_'
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => {
                println!("Error reading directory");
                return TodoListTopic::new(); 
            }
        };
        // For each name in the list, read the content and add it to the items hasmap, where key is the prefix of the file
        let mut todo_list_topic = TodoListTopic::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if let Some(topic) = file_name.split("_").next() {
                        println!("Found topic : {}", topic);
                        todo_list_topic.items.insert((*topic).to_string(), TodoList::load_from_file((*  topic).to_string()));
                    }
                }
            }
        }
        return TodoListTopic::new()
    }
}


fn main() {
    let name: String = "test".to_string();
    let mut todo_list = TodoList::load_from_file(name);
    let current_topic =  String::new();

    loop {
        print!("Enter command (add/complete/list/change/quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "add" => {
                print!("Enter todo description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add(description.trim().to_string());
                println!("Todo added successfully!");
            }
            "complete" => {
                print!("Enter todo ID to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<usize>() {
                    if todo_list.mark_complete(id) {
                        println!("Todo marked as complete!");
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID. Please enter a number.");
                }
            }
            "list" => {
                println!("Current TODO list:");
                // if current_topic is an empty string then display all topics and tasks
                todo_list.display();
            }
            // "change" => {
            //     println!("Current Topics:");
            //     todo_list.display_topics();
            //     println!("Choose existing or create a new one.")
            //     io::stdin().read_line(&mut current_topic).unwrap();
            // }
            // "clear" => {
            //     // if all tasks are marked completed, then delete the topic
            //     // else remove the items if they are marked as completed
            //     todo_lsit.clear_topic(current_topic)
            //     println!("Topic {} cleared", current_topic)
            // }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command. Please try again."),
        }
    }
}


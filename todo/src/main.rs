use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;
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


// impl TodoListTopic {
//     fn new() -> Self {
//         TodoListTopic {
//             items: HashMap::new(),
//             current_topic: "",
//         }
//     }

//     fn add(&mut self, description: String) {
//         let todo = Todo {
//             description,
//             completed: false,
//         };
//         self.items.get_mut(&self.current_topic).unwrap().items.insert(self.current_id, todo);
//         self.save_to_file();
//     }

//     fn mark_complete(&mut self, id: usize) -> bool {
//         if let Some(todo) = self.items.get_mut(self.current_topic).items.get_mut(&id) {
//             todo.completed = true;
//             self.save_to_file();
//             true
//         } else {
//             false
//         }
//     }

//     fn display(&self) {
//         println!("{} :", self.current_topic);
//         for (id, todo) in &self.get_mut(&self.current_topic).items {
//             let status = if todo.completed { "[x]" } else { "[ ]" };
//             println!("{} {}: {}", id, status, todo.description);
//         }
//     }

//     fn save_to_file(&self) {
//         self.items.get_mut(&self.current_topic).save_to_file();
//         // let serialized = serde_json::to_string(self).unwrap();
//         // fs::write("todo_list.json", serialized).expect("Unable to write file");
//     }

//     fn load_from_file() -> Self {
//         // Read all of the topics : create folder to gather them
//         match fs::read_to_string("todo_list.json") {
//             Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| TodoList::new()),
//             Err(_) => TodoList::new(),
//         }
//     }
// }


fn main() {
    let name: String = "test".to_string();
    let mut todo_list = TodoList::load_from_file(name);
    // let current_topic =  String::new();

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


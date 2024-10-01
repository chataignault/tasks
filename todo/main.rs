use std::collections::HashMap;
use std::io::{self, Write};

struct Todo {
    description: String,
    completed: bool,
}

struct TodoList {
    items: HashMap<usize, Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
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
    }

    fn mark_complete(&mut self, id: usize) -> bool {
        if let Some(todo) = self.items.get_mut(&id) {
            todo.completed = true;
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
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("Enter command (add/complete/list/quit): ");
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
                todo_list.display();
            }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command. Please try again."),
        }
    }
}


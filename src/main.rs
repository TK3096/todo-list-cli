use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    created_at: String
}

fn main() {
    println!("Welcome to the Task Manager!\n");
    
    loop {
        println!("### Please choose an option:");
        println!("1. Create a new task");
        println!("2. View all tasks");
        println!("3. Mark task as completed");
        println!("4. Delete a task");
        println!("5. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                continue;
            }
        };

        match choice {
            1 => {
                create_task();
            },
            2 => {
                print_tasks();
            }
            3 => {
                mark_task_completed();
            }
            4 => {
                delete_task();
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

fn create_task() {
    println!("Enter task description:");

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    let mut tasks = load_tasks();

    let task = Task {
        id: tasks.len() as u32 + 1,
        description: description.trim().to_string(),
        completed: false,
        created_at: chrono::Local::now().to_string()
    };

    tasks.push(task);
    save_tasks(&tasks);
    println!("Task created successfully!\n");
}

fn print_tasks() {
    let tasks = load_tasks();

    if !tasks.is_empty() {
        for task in tasks.iter() {
            let completed = if task.completed { "Completed" } else { "Not Completed" };
            println!("[{}] {} - {} ({})", task.id, task.description, completed, task.created_at);
        }
        println!("\nTotal tasks: {}\n", tasks.len());
    } else {
        println!("No tasks found.\n");
    }
}

fn mark_task_completed() {
    let mut tasks = load_tasks();

    if tasks.is_empty() {
        println!("No tasks to mark as completed.\n");
        return;
    }

    println!("Enter the ID of the task to mark as completed:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid task ID.");
            return;
        }
    };

    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            break;
        }
    }

    save_tasks(&tasks);
    println!("Task marked as completed!\n");
}

fn delete_task() {
    let mut tasks = load_tasks();

    if tasks.is_empty() {
        println!("No tasks to delete.\n");
        return;
    }

    println!("Enter the ID of the task to delete:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid task ID.");
            return;
        }
    };

    tasks.retain(|task| task.id != id);
    save_tasks(&tasks);
    println!("Task deleted successfully!\n");
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(&tasks);

   match json {
        Ok(data) => {
            match File::create("data/tasks.json") {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(data.as_bytes()) {
                        eprintln!("Error writing to file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error serializing tasks: {}", e);
        }
    }
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("data/tasks.json") {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(tasks) => tasks,
                Err(e) => {
                    eprintln!("Error deserializing tasks: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Vec::new()
        }
    }
}
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    is_completed: bool,
}

const JSON_FILE_NAME: &str = "tasks.json";

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("To-do List Menu:");
        println!("1. Add task");
        println!("2. View Tasks");
        println!("3. Mark tash as completed");
        println!("4. Delete task");
        println!("5. Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_as_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("All tasks saved, GoodBye");
                break;
            }

            _ => println!("Invalid choice. Please try again!"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{prompt}");

    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(JSON_FILE_NAME) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create(JSON_FILE_NAME).expect("Failed to save tasks");

    file.write_all(json.as_bytes())
        .expect("failed to write tasks to file");
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description");
    let id = tasks.len() + 1;

    tasks.push(Task {
        id,
        description: description.trim().to_string(),
        is_completed: false,
    });

    println!("Task added");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            let status = if task.is_completed {
                "Completed"
            } else {
                "Not completed"
            };
            println!("{} - {}: {}", task.id, status, task.description);
        }
    }
}

fn mark_as_completed(tasks: &mut [Task]) {
    let id = get_input("Enter task ID to mark as completed: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.is_completed = true;
        } else {
            println!("Task not found");
        }
    } else {
        println!("Invalid ID");
    };
}

// TOOD: check this;
fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as completed: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|task| task.id == id) {
            tasks.remove(index);
            println!("Task deleted");
        } else {
            println!("Task not found");
        }
    } else {
        println!("Invalid ID");
    };
}

use std::io;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Running,
    Failed,
    Completed,
}

struct Task {
    name: String,
    objective: String,
    status: Status,
}

impl Task {
    fn new(name: &str, objective: &str) -> Self {
        Task {
            name: name.to_string(),
            objective: objective.to_string(),
            status: Status::Pending,
        }
    }
}

fn get_input() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}

fn create_task() -> Task {
    println!("Enter task name:");

    let mut name: String = String::new();
    name = get_input();

    let mut obj: String = String::new();
    obj = get_input();

    let task = Task::new(name.trim(), obj.trim());
    println!("Task {} successfully created. Objective: {} Status: {:?}", name.trim(), obj.trim(), Status::Pending);

    task
}

fn show_tasks(task_vec: &Vec<Task>) -> () {
    let mut counter: usize = 1;

    for task in task_vec {

        println!("Task #{}: {}\nObjective: {}\nStatus: {:?}\n", counter, task.name, task.objective, task.status);
        counter = counter + 1;

    }
}

fn change_task(task_vec: &mut Vec<Task>) -> () {
    println!("Enter the task number you want to change:");

    let mut task_number: String = String::new();

    task_number = get_input();

    if task_number.trim().parse::<usize>().is_ok() {
        let task_number: usize = task_number.trim().parse::<usize>().unwrap();

        println!("Enter the new status (pending, running, failed, completed):");

        let mut new_status: String = String::new();

        new_status = get_input();

        match new_status.trim() {
            "pending" => {

                task_vec[task_number - 1].status = Status::Pending;

            }

            "running" => {

                task_vec[task_number - 1].status = Status::Running;

            }
            
            "failed" => {
                task_vec[task_number - 1].status = Status::Failed;

            }

            "completed" => {

                task_vec[task_number - 1].status = Status::Completed;

            }

            _ => {

                println!("Invalid status. Changing the status to pending.");
                task_vec[task_number - 1].status = Status::Pending;

            }
        }

        if task_vec.is_empty() {

            println!("No tasks to change.");

        } else if task_number > task_vec.len() {

            println!("Invalid task number.");

        } else {

            println!("Task #{} status successfully changed to {}.", task_number, new_status.trim());

        }
    }
}

fn save_tasks(task_vec: &Vec<Task>) -> () {
    let mut file: File = File::create("tasks.txt").expect("Failed to create file.");

    for task in task_vec {

        if !task_vec.is_empty() {

            let task_string: String = format!("Task: {}\nObjective: {}\nStatus: {:?}\n\n", task.name, task.objective, task.status);
            file.write_all(task_string.as_bytes()).expect("Failed to write to file.");

        } else {

            println!("No tasks to save.");

        }
    } 
}

fn main() -> () {
    let mut task_vec: Vec<Task> = Vec::new();

    loop {
        println!("Enter a command (create, show, change, save, exit):");

        let mut command: String = String::new();

        command = get_input();

        match command.trim() {
            "create" => {

                let task = create_task();
                task_vec.push(task);
            }

            "show" => {

                show_tasks(&task_vec);
            }

            "change" => {

                change_task(&mut task_vec);
            }

            "save" => {
                save_tasks(&mut task_vec);
            }

            "exit" => {

                break;
            }

            _ => {

                println!("Invalid command.\nTry Again (create, show, change, save, exit):");
            }
            
        }
    }
}
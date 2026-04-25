use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write, stdin, stdout};
use std::process::Command;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    is_done: bool,
}
impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            is_done: false,
        }
    }
    fn change_status(&mut self) {
        self.is_done = !self.is_done
    }
}
pub fn todoapp() {
    let path = "assets/tasks.json";
    println!("welcome to todo application");
    loop {
        println!("\nchoose an option:");
        println!(
            "\tA-Show tasks \n\tB-Add new task\n\tC-Delete task\n\tD-Change tasks status\n\tE-Quit"
        );
        print!("your input?: ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let tasks = read_tasks(path);
        match input.trim().to_lowercase().as_str() {
            "a" => {
                show_tasks(&tasks);
                continue;
            }

            "b" => {
                let mut tasks = read_tasks(path);
                add_task(&mut tasks);
                write_tasks(&tasks, path);
                continue;
            }
            "c" => {
                let mut tasks = read_tasks(path);
                delete_task(&mut tasks);
                write_tasks(&tasks, path);
                continue;
            }
            "d" => {
                let mut tasks = read_tasks(path);
                show_tasks(&tasks);
                mark_task(&mut tasks);
                write_tasks(&tasks, path);
                continue;
            }
            "e" => break,
            _ => {
                println!("choose a valid option!");
            }
        }
    }
}
fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("ERROR: failed to clear terminal");
}
fn write_tasks(tasks: &[Task], path: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("ERROR : error dealing with the file path");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &tasks)
        .expect("ERROR: failed to serialize and write tasks");
    // insure all the data in memory is pushed to the disk
    writer
        .flush()
        .expect("ERROR: failed to flush the data from memory")
}
fn read_tasks(path: &str) -> Vec<Task> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .expect("ERROR : error dealing with the file path");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| {
        println!("you have no tasks , create new one");
        Vec::new()
    })
}
fn show_tasks(tasks: &[Task]) {
    clear_terminal();
    println!("your tasks: ");
    tasks.iter().for_each(|task| {
        let status = match task.is_done {
            true => "Done",
            false => "Undone",
        };
        println!("{} - {} : {}", task.id, task.description, status)
    })
}
fn add_task(tasks: &mut Vec<Task>) {
    clear_terminal();
    print!("task description: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("ERROR: failed to read from input ");
    let task = Task::new(tasks.len() + 1, input.trim().to_string());
    tasks.push(task);
}
fn delete_task(mut tasks: &mut Vec<Task>) {
    print!("select tasks to delete [1,2,3..]: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("ERROR: failed to read input");
    match input.trim().to_lowercase().parse::<usize>() {
        Ok(i) => {
            let _ = tasks.remove(i - 1);
            rearrange(&mut tasks);
            println!("your task is deleted.");
            show_tasks(tasks)
        }
        Err(_) => println!("you entered invalid task"),
    }
}
fn rearrange(tasks: &mut Vec<Task>) {
    let mut counter: usize = 1;
    tasks.iter_mut().for_each(|task| {
        task.id = counter;
        counter += 1;
    });
}
fn mark_task(tasks: &mut Vec<Task>) {
    print!("select tasks to change status [1,2,3..]: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("ERROR: failed to read input");
    match input.trim().to_lowercase().parse::<usize>() {
        Ok(i) => {
            tasks[i - 1].change_status();
            println!("your task changed.");
            show_tasks(tasks)
        }
        Err(_) => println!("you entered invalid task"),
    }
}

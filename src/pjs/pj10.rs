use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, ErrorKind, Seek, SeekFrom, Write, stdin, stdout};

use std::process::{Command, ExitStatus};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}
#[derive(Serialize, Deserialize, Debug)]

enum TaskStatus {
    Done,
    Planned,
    Abandoned,
}
impl TaskStatus {
    fn to_string(&self) -> &str {
        match self {
            TaskStatus::Planned => "Planned",
            TaskStatus::Done => "Done",
            TaskStatus::Abandoned => "Aboandoned",
        }
    }
}
pub fn todo_app() -> io::Result<()> {
    println!("welcome to todo app , what u need to do (select A,B...) : ");
    let file = File::open("assets/tasks.json")
        .unwrap_or_else(|e| File::create("assets/tasks.json").unwrap());
    let mut reader = BufReader::new(&file);
    loop {
        // define what user need to do
        // show tasks
        // delete task
        // change task status
        // add task
        // back action
        // quit
        let mut input = String::new();

        println!("\tA)-Show tasks.\n\tB)-Add task.\n\tC)-Delete Task.\n\tD)-Quit the app.");
        print!("input: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "a" => {
                show_tasks(&mut reader);
            }
            "b" => add_tasks(&mut reader),
            "c" => delete_tasks(&mut reader),
            "d" => break,
            _ => {
                println!("invalid input, try again.");
                continue;
            }
        }
    }
    Ok(())
}

fn show_tasks(reader: &mut BufReader<&File>) {
    reader.seek(SeekFrom::Start(0)).unwrap();
    let tasks: Vec<Task> = serde_json::from_reader(reader).unwrap_or_else(|e| Vec::new());
    tasks.iter().for_each(|task| {
        println!(
            "{}- {} : {} ",
            task.id,
            task.description,
            task.status.to_string()
        )
    });
}
fn add_tasks(reader: &mut BufReader<&File>) {
    clear_terminal().unwrap();
    reader.seek(SeekFrom::Start(0)).unwrap();

    let mut tasks: Vec<Task> = serde_json::from_reader(reader).unwrap_or_else(|e| {
        println!("no tasks , adding new one....");
        Vec::new()
    });

    print!("enter the description of the task: ");
    stdout().flush().expect("error");
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();
    let id = tasks.len() + 1;
    let task = Task {
        description: description.trim().to_string(),
        status: TaskStatus::Planned,
        id,
    };
    tasks.push(task);
    save_tasks(tasks).unwrap();
}
fn delete_tasks(reader: &mut BufReader<&File>) {
    reader.seek(SeekFrom::Start(0)).unwrap();
    clear_terminal().unwrap();
    show_tasks(reader);
    reader.seek(SeekFrom::Start(0)).unwrap();

    print!("Select id of the tasks to delete: ");
    stdout().flush().unwrap();
    let mut id_string = String::new();
    stdin().read_line(&mut id_string).unwrap();
    let id = id_string.trim().to_lowercase().parse::<usize>().unwrap();
    let mut tasks :Vec<Task>= serde_json::from_reader(reader).unwrap();
    tasks.remove(id-1);
    save_tasks(tasks).unwrap();
}
fn change_status() {}
fn save_tasks(tasks: Vec<Task>) -> Result<bool, ErrorKind> {
    let file = match File::create("assets/tasks.json") {
        Ok(f) => f,
        Err(_) => {
            println!("error writing a file");
            return Err(ErrorKind::NotFound);
        }
    };
    let mut writer = BufWriter::new(file);
    let tasks_string = serde_json::to_string(&tasks).unwrap();
    write!(writer, "{}", tasks_string).unwrap();
    Ok(true)
}
fn clear_terminal() -> io::Result<ExitStatus> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status()
    } else {
        Command::new("clear").status()
    }
}

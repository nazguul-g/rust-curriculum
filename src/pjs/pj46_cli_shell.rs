use std::env;
use std::io::{Stdin, Write, stdin, stdout};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn cli_shell() {
    println!("DIY cli shell interface");
    let separator = "-".repeat(80);
    loop {
        let current_path = env::current_dir()
            .unwrap_or_else(|_| Path::new("?").to_path_buf())
            .display()
            .to_string();

        let mut input = String::new();
        print!("rust-shell:{}> ", current_path);
        stdout().flush().unwrap();
        if stdin().read_line(&mut input).is_err() {
            println!("{separator}");
            continue;
        };
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input.starts_with("cd") {
            let path = input.strip_prefix("cd").unwrap().trim();
            if path.is_empty() {
                if let Ok(home) = env::var("HOME") {
                    let _ = env::set_current_dir(home);
                    println!("{separator}");
                    continue;
                };
            }
            if let Err(e) = env::set_current_dir(path) {
                println!("{path} is not a path, {e}");
            }
            println!("{separator}");
            continue;
        }
        if input == "exit" {
            break;
        }
        if input == "pwd" {
            println!("{current_path}");
            continue;
        }

        run_command(&input);
    }
}
fn run_command(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let (cmd, args) = parts.split_first().unwrap();
    match Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                println!("failed to fork-exec,{e}");
                return;
            }
        }
        Err(e) => {
            println!("failed to fork-exec,{e}");
            return;
        }
    }
}

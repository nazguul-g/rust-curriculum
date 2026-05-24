use colored::Colorize;
use std::io::{self, Write};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

const MAX_PROCESSES: usize = 20;

pub fn process_manager() {
    // Initialize system state with process tracking enabled
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    loop {
        // Always refresh process data before rendering views
        sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything(),
        );

        clear_screen();
        render_header();

        println!("{}", " Please select an option:".bold());
        println!("  [{}] List top processes by CPU", "1".cyan());
        println!("  [{}] Search for a process by name", "2".cyan());
        println!("  [{}] Kill a process by PID", "3".cyan());
        println!("  [{}] Exit", "4".red());
        println!();

        let choice = prompt("Enter choice (1-4): ");

        match choice.as_str() {
            "1" => {
                show_processes(&sys);
                pause();
            }
            "2" => {
                search_process(&sys);
                pause();
            }
            "3" => {
                kill_process(&mut sys);
                pause();
            }
            "4" => {
                println!("{}", "\nExiting Process Manager. Goodbye!".green());
                break;
            }
            _ => {
                println!("{}", "\nInvalid option, please try again.".red());
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}

fn render_header() {
    println!("{}", "==================================================".magenta());
    println!("{}", "           SYSTEM PROCESS MANAGER v1.0            ".bold().magenta());
    println!("{}", "==================================================".magenta());
    println!();
}

fn clear_screen() {
    // Clear screen and reset cursor to top-left using ANSI escape codes
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    io::stdout().flush().unwrap();
}

fn pause() {
    print!("\nPress Enter to return to the main menu...");
    io::stdout().flush().unwrap();
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

fn prompt(msg: &str) -> String {
    print!("{}", msg.bold());
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn show_processes(sys: &System) {
    clear_screen();
    render_header();
    
    let mut processes: Vec<_> = sys.processes().values().collect();
    
    // Sort descending by CPU usage
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    // Table Header
    println!(
        "{:<6} {:<10} {:<25} {:<12} {:<12}",
        "Idx", "PID", "Name", "CPU %", "Memory"
    );
    println!("{}", "-".repeat(65).dimmed());

    // Prevent out-of-bounds if total running processes are less than MAX_PROCESSES
    let limit = std::cmp::min(processes.len(), MAX_PROCESSES);

    for (i, process) in processes.iter().enumerate().take(limit) {
        // Convert KB to MB safely
        let process_memory = process.memory() / 1024 / 1024; 
        let name = process.name().to_string_lossy();
        // Truncate long names to keep columns perfectly aligned
        let truncated_name = if name.len() > 22 {
            format!("{}...", &name[0..19])
        } else {
            name.to_string()
        };

        println!(
            "{:<6} {:<10} {:<25} {:<12.2} {:<12} MB",
            i + 1,
            process.pid().to_string().cyan(),
            truncated_name.bright_green(),
            process.cpu_usage(),
            process_memory
        );
    }
}

fn search_process(sys: &System) {
    clear_screen();
    render_header();

    let target_name = prompt("Enter the name of the process to search: ").to_lowercase();
    if target_name.is_empty() { return; }

    println!("\n{:<10} {:<30} {:<10}", "PID", "Name", "Status");
    println!("{}", "-".repeat(55).dimmed());

    let mut found_any = false;
    for (pid, process) in sys.processes() {
        let proc_name = process.name().to_string_lossy().to_lowercase();
        
        // Using 'contains' is a vastly superior UX compared to an exact match
        if proc_name.contains(&target_name) {
            found_any = true;
            println!(
                "{:<10} {:<30} {:<10}",
                pid.to_string().cyan(),
                process.name().to_string_lossy().bright_green(),
                "Running".green()
            );
        }
    }

    if !found_any {
        println!("{}", "No matching processes found.".yellow());
    }
}

fn kill_process(sys: &mut System) {
    clear_screen();
    render_header();

    let pid_str = prompt("Enter the PID of the process to terminate: ");
    
    if let Ok(pid_u32) = pid_str.parse::<u32>() {
        let target_pid = Pid::from_u32(pid_u32);
        
        // Safely check if the process exists in our sysinfo snapshot
        if let Some(process) = sys.process(target_pid) {
            print!("Are you sure you want to kill {} (PID: {})? (y/N): ", process.name().to_string_lossy().red(), pid_u32);
            io::stdout().flush().unwrap();
            
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();
            
            if confirm.trim().to_lowercase() == "y" {
                // sysinfo's native kill function API execution
                if process.kill() {
                    println!("{}", format!("\nSuccessfully sent termination signal to PID {}.", pid_u32).green());
                } else {
                    println!("{}", format!("\nFailed to terminate PID {}. You might lack permissions.", pid_u32).red());
                }
            } else {
                println!("\nOperation cancelled.");
            }
        } else {
            println!("{}", format!("\nNo active process found with PID {}.", pid_u32).yellow());
        }
    } else {
        println!("{}", "\nInvalid PID format. Please enter a valid number.".red());
    }
}
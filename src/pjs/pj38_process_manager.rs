// Queries system processes using the sysinfo crate,
// providing options to list all active processes, search for processes by name,
// and terminate processes using their PID.

// thoughts
// i know how to show all processes, but this is a bad ux , we need to set a limit
// how do i kill processes. is there api, or do i need to use hardcoded terminal commands
//

use std::io::Write;
use std::time::Duration;
use std::{io, thread};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
const MAX_PROCESSES: usize = 30;
pub fn process_manager() {
    // we only refresh the processes, we dont wannt overkill
    // only one call , we dont need to call this on each function
    let mut sys = sysinfo::System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    loop {
        show_processes(&mut sys);
    }
}
fn kill_process() {}
fn search_process(sys: System) {
    let mut processes: Vec<_> = sys
        .processes()
        .iter()
        .map(|(pid, process)| process)
        .collect();
    let name = prompt("enter the name of process: ");
    
}
fn show_processes(sys: &mut System) {
    let mut processes: Vec<_> = sys
        .processes()
        .iter()
        .map(|(pid, process)| process)
        .collect();
    // we want to save space of the terminal , each refresh takes no extra space
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
    for i in 0..MAX_PROCESSES {
        let process = processes[i];
        let process_memory = process.virtual_memory() / (1024 * 2);
        // needs more regulations
        println!(
            "p{}\tpid: {}\t name: {}\t cpu usage: {:.2}%\t memory usage: {}MB",
            i,
            process.pid().to_string(),
            process.name().to_string_lossy(),
            process.cpu_usage(),
            process_memory
        );
    }
    // move the cursor up again
    for _ in 0..MAX_PROCESSES {
        print!("\r");
    }
    thread::sleep(Duration::from_secs(15));
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

// Monitors system diagnostics using the sysinfo crate, periodically printing core CPU usage, total memory consumption, process counts,
// and the top 5 active processes.

use std::thread;
use std::time::Duration;

pub fn system_monitor() {
    let mut sys = sysinfo::System::new_all();

    loop {
        sys.refresh_all();
        let memory_consumption = sys.used_memory() / (1024 * 2);
        let total_memory = sys.total_memory() / (1024 * 2);
        for (i , cpu) in sys.cpus().iter().enumerate() {

            println!("cpu {}usage: {:.2}%", i , cpu.cpu_usage())

        }
        println!("memory used : {}", memory_consumption);
        println!("total memory : {}", total_memory);
        println!("total cpu usage: {:.2}%", sys.global_cpu_usage());
        println!("total processes: {}", sys.processes().len());
        println!("top 5 processes");
        let mut processes:Vec<_> = sys.processes().values().collect();
        processes.sort_by(|a,b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        for proc in processes.iter().take(10) {
            println!(
                "PID: {:<6} CPU: {:>5.1}%  Name: {}",
                proc.pid(),
                proc.cpu_usage(),
                proc.name().to_string_lossy()
            );
        }

        thread::sleep(Duration::from_secs(30));

    }
}

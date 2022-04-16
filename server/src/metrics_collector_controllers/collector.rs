use std::thread;
use std::time;
use crate::metrics_collector_controllers::collector_utils;
use procfs::process::Process;
use procfs::ticks_per_second;
use collector_utils::Proc;

const SAMPLE_TIME: u64 = 1;

pub fn collect_all_metrics() -> Vec<Proc> {
    // Collect Process Info
    let mut processes = Vec::new();
    for p in procfs::process::all_processes().unwrap() {
        // Use default constructor to create "null" process
        let mut new_process = Proc::default();

        // get memory metrics from get_memory_usage
        let memory_info = get_memory_usage(p);

        // set process object's fields to collected metrics
        new_process.set_pid(memory_info.0);
        new_process.set_pname(memory_info.1);
        new_process.set_threads(memory_info.2);
        new_process.set_pmemory(memory_info.3);

        processes.push(new_process);
    }
    // print_processes(processes);
    return processes;
}

pub fn get_memory_usage(p: procfs::process::Process) -> (i32, String, i64, String){
    let id = p.pid;
    let p_memory = p.stat.rss_bytes().unwrap();
    let p_name = p.stat.comm;
    let num_threads = p.stat.num_threads;
    let mem_usage = collector_utils::format_memory(p_memory);


    let memory_info: (i32, String, i64, String) = (id, p_name, num_threads, mem_usage);

    return memory_info;
}

pub fn get_disk_usage(p: procfs::process::Process) {
    // TODO: Format variables below (format_memory function)
    let read = p.io().unwrap().read_bytes;
    let written = p.io().unwrap().write_bytes;
}

pub fn get_cpu_usage(p: Process) -> u64 {
    // Get ticks per second for calculating CPU time.
    let ticks_per_second = ticks_per_second().unwrap() as u64;

    // Get amount of time p has been scheduled in kernel mode and user mode at
    // this moment.
    let kernel_mode_time_before = p.stat.stime / ticks_per_second;
    let user_mode_time_before = p.stat.utime / ticks_per_second;

    println!("Kernel mode time before sample: {}", kernel_mode_time_before);
    println!("User mode time before sample: {}", user_mode_time_before);

    // Let the sample time pass.
    thread::sleep(time::Duration::from_secs(SAMPLE_TIME));

    // Get amount of time p has been scheduled in kernel mode and user mode
    // again.
    let kernel_mode_time_after = p.stat.stime / ticks_per_second;
    let user_mode_time_after = p.stat.utime / ticks_per_second;

    println!("Kernel mode time after sample: {}", kernel_mode_time_after);
    println!("User mode time after sample: {}", user_mode_time_after);

    // Calculate total time in both modes.
    let kernel_mode_time = kernel_mode_time_after - kernel_mode_time_before;
    let user_mode_time = user_mode_time_after - user_mode_time_before;

    println!("Kernel mode time: {}", kernel_mode_time);
    println!("User mode time: {}", user_mode_time);

    // Calculate total CPU usage over the sample time.
    let cpu_usage = ((kernel_mode_time + user_mode_time) / SAMPLE_TIME) * 100;

    println!("CPU usage: {}", cpu_usage);

    // Send back the total CPU usage.
    return cpu_usage;
}

#[cfg(test)]
mod collector_tests {
    use super::*;

    // Ravosa Tests
    #[test]
    fn cpu_usage() {
        // Check this program's process ID.
        let this_process = Process::myself().unwrap();

        // Get the cpu usage of this process.
        let result = get_cpu_usage(this_process);

        // Get CPU usage of this process.
        assert!(result > 0);
    }
}
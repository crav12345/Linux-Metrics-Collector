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

#[cfg(test)]
mod collector_tests {

    // Test to make sure that the format_memory() function returns the expected values
    #[test]
    fn test_get_memory_usage() {
        // get process
        let p1 = procfs::process::all_processes().unwrap();
        let p2 = p1.first().unwrap();
        let p3 = p2.to_owned();
        let result = crate::collector::get_memory_usage(p3);

        // Make sure that the returned metrics have values that make sense
        assert!(result.0.is_positive());
        assert!(result.1.len() > 0);
        assert!(result.2 >= 0);
        assert!(result.3.len() > 2);
    }
}
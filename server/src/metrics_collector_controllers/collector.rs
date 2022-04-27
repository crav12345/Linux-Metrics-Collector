use std::thread;
use std::time;
use procfs::process::Process;
use procfs::{sys, ticks_per_second};
use sysinfo::*;
use sysinfo::Signal::Sys;
use crate::metrics_collector_controllers::collector_utils;
use crate::metrics_collector_controllers::structs::Proc;
use crate::database::{get_cpu_usage_by_pid, get_current_metrics_from_db};
use crate::format_percent_usage;

const SAMPLE_TIME: f32 = 15.0;

pub fn collect_all_metrics(is_first_interval: bool) -> Vec<Proc> {
    let mut sys = sysinfo::System::new_all();
    let mut disks = sys.disks();
    let mut disk_space = 0;
    for disk in disks {
        disk_space += disk.available_space();
    }

    // Collect Process Info
    let mut processes = Vec::new();
    for p in procfs::process::all_processes().unwrap() {
        // Use default constructor to create "null" process
        let mut new_process = Proc::default();

        let cpu_usage = collect_cpu_usage(&p, is_first_interval);
        let disk_usage = collect_disk_usage(&p, disk_space);

        // get memory metrics from get_memory_usage
        let memory_info = collect_memory_usage(p);

        // set process object's fields to collected metrics
        new_process.set_pid(memory_info.0);
        new_process.set_pname(memory_info.1);
        new_process.set_threads(memory_info.2);
        new_process.set_pmemory(memory_info.3);
        new_process.set_cpu_usage(cpu_usage.0);
        new_process.set_kernel_mode_time(cpu_usage.1);
        new_process.set_user_mode_time(cpu_usage.2);
        new_process.set_disk_usage(disk_usage);

        processes.push(new_process);
    }


    println!("Database Updated");

    return processes;
}

pub fn collect_memory_usage(p: procfs::process::Process) -> (i32, String, i64, String){
    let id = p.pid;
    let p_memory = p.stat.rss_bytes().unwrap();
    let p_name = p.stat.comm;
    let num_threads = p.stat.num_threads;
    let mem_usage = collector_utils::format_memory(p_memory);

    let memory_info: (i32, String, i64, String) = (id, p_name, num_threads, mem_usage);

    return memory_info;
}

// TODO: May need to do this over an interval because you get > 100% usage.
// TODO: Occasionally a process is not found which causes a crash. Process is likely terminated in middle of method call.
pub fn collect_disk_usage(p: &procfs::process::Process, disk_space: u64) -> String {
    // Determine how much space this process is using.
    let read = p.io().unwrap().read_bytes as f32;
    let written = p.io().unwrap().write_bytes as f32;

    // Calculate disk usage of this process as a percentage.
    let total_bytes = read + written;
    let disk_usage = (total_bytes / (disk_space as f32)) * 100.0;

    // Kick the percentage of use back up.
    return format_percent_usage(disk_usage);
}

// TODO: Make tests to see if it works with both first interval and all others.
// TODO: Total usage is > 100% for some early intervals. May need to use is_first_interval.
pub fn collect_cpu_usage(p: &procfs::process::Process, is_first_interval: bool) -> (String, f32, f32) {
    // Get ticks per second for calculating CPU time.
    let ticks_per_second = ticks_per_second().unwrap() as f32;

    // Get how many times the CPU has ticked in 15 seconds.
    let cpu_time_over_interval = ticks_per_second * SAMPLE_TIME;

    // Amount of time in kernel mode at last interval.
    let mut kernel_mode_time_before: f32 = 0.0;
    let mut user_mode_time_before: f32 = 0.0;

    // Amount of time in kernel mode and user mode now.
    let mut kernel_mode_time_now: f32 = 0.0;
    let mut user_mode_time_now: f32 = 0.0;

    // Total time in each mode individually.
    let mut kernel_mode_time: f32 = 0.0;
    let mut user_mode_time: f32 = 0.0;

    // Total time in both user and kernel mode.
    let mut total_mode_time: f32 = 0.0;

    // Total usage.
    let mut cpu_usage: f32 = 0.0;

    // String to be returned with CPU info.
    let mut cpu_usage_description: String = "LOADING".to_owned();

    // Only check usage if at least one sample interval has passed.
    if !is_first_interval {
        // Query database for this process' user mode and kernel mode time 15 seconds ago.
        let old_cpu_usage = get_cpu_usage_by_pid(p.pid).unwrap();
        if old_cpu_usage.len() > 0 {
            kernel_mode_time_before = old_cpu_usage[0];
            user_mode_time_before = old_cpu_usage[1];
        }

        // Get amount of time p has been scheduled in kernel mode and user mode
        // since the last sample.
        kernel_mode_time_now = p.stat.stime as f32;
        user_mode_time_now = p.stat.utime as f32;

        // Calculate total time in both modes and find their sum.
        kernel_mode_time = kernel_mode_time_now - kernel_mode_time_before;
        user_mode_time = user_mode_time_now - user_mode_time_before;
        total_mode_time = kernel_mode_time + user_mode_time;

        // Calculate total CPU usage over the sample time.
        cpu_usage = (total_mode_time / cpu_time_over_interval)  as f32 * 100.0;

        // Update description to reflect usage as a percent if it is accurate.
        if cpu_usage <= 100.00 {
            cpu_usage_description = format_percent_usage(cpu_usage);
        }
    }

    return (cpu_usage_description, kernel_mode_time_now, user_mode_time_now);
}

// TODO: get_network_usage() method and tests.

#[cfg(test)]
mod collector_tests {
    use sysinfo::{DiskExt, SystemExt};
    use crate::collector::{collect_disk_usage, collect_cpu_usage, collect_memory_usage};

    #[test]
    fn cpu_usage() {
        use procfs::process::Process;
        // Check this program's process ID.
        let this_process = procfs::process::Process::myself().unwrap();

        // Get the cpu usage of this process.
        let result_vector = collect_cpu_usage(&this_process, false);

        if result_vector.0 != "LOADING" {
            // Get the amount of CPU usage and convert it to an f32 for comparison.
            let percent_usage = result_vector.0.replace("%", "");
            let result = percent_usage.parse::<f32>().unwrap();

            assert!(result <= 100.0);
        } else {
            // Validate result.
            assert_eq!(result_vector.0, "LOADING");
        }
    }

    #[test]
    fn disk_usage() {
        let mut sys = sysinfo::System::new_all();
        let mut disks = sys.disks();
        let mut disk_space = 0;
        for disk in disks {
            disk_space += disk.available_space();
        }

        // Check this program's process ID.
        let this_process = procfs::process::Process::myself().unwrap();

        // Get the cpu usage of this process.
        let percent_usage = collect_disk_usage(&this_process, disk_space).replace("%","");

        // Convert the percent usage string to a float.
        let result = percent_usage.parse::<f32>().unwrap();

        // Validate result.
        assert!(result <= 100.0);
    }

    // Test to make sure that the format_memory() function returns the expected values
    #[test]
    fn test_collect_memory_usage() {
        // get process
        let p1 = procfs::process::all_processes().unwrap();
        let p2 = p1.first().unwrap();
        let p3 = p2.to_owned();
        let result = crate::collector::collect_memory_usage(p3);

        // Make sure that the returned metrics have values that make sense
        assert!(result.0.is_positive());
        assert!(result.1.len() > 0);
        assert!(result.2 >= 0);
        assert!(result.3.len() > 2);
    }
}

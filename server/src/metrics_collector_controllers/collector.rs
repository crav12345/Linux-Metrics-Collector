use std::thread;
use std::time;
use actix_web::rt::System;
use crate::metrics_collector_controllers::collector_utils;
use procfs::process::Process;
use procfs::{sys, ticks_per_second};
use sysinfo::*;
use sysinfo::Signal::Sys;
use collector_utils::Proc;
use crate::format_percent_usage;

const SAMPLE_TIME: i64 = 500;

pub fn collect_all_metrics() -> Vec<Proc> {
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

        // CPU info requires its own thread because it needs to measure usage
        // over a sample time of 's' seconds. This causes a bottleneck if left
        // on the same thread.
        //let handler = thread::spawn(|| {
        //    return get_cpu_usage(&p);
        //});
        // Just made cpu sample time extremely small for now.
        let cpu_usage = get_cpu_usage(&p);
        let disk_usage = get_disk_usage(&p, disk_space);

        // get memory metrics from get_memory_usage
        let memory_info = get_memory_usage(p);

        // set process object's fields to collected metrics
        new_process.set_pid(memory_info.0);
        new_process.set_pname(memory_info.1);
        new_process.set_threads(memory_info.2);
        new_process.set_pmemory(memory_info.3);
        new_process.set_cpu_usage(cpu_usage);
        //new_process.set_cpu_usage(handler.join().unwrap());
        new_process.set_disk_usage(disk_usage);

        processes.push(new_process);
    }

    println!("Done");

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

pub fn get_disk_usage(p: &procfs::process::Process, disk_space: u64) -> String {
    // Determine how much space this process is using.
    let read = p.io().unwrap().read_bytes as f32;

    let written = p.io().unwrap().write_bytes as f32;

    // Calculate disk usage of this process as a percentage.
    let total_bytes = read + written;
    let disk_usage = (total_bytes / (disk_space as f32)) * 100.0;

    // Kick the percentage of use back up.
    return format_percent_usage(disk_usage);
}

pub fn get_cpu_usage(p: &procfs::process::Process) -> String {
    // TODO: A way to do this without another thread would be to just check difference w/ 15 second intervals.

    // Get ticks per second for calculating CPU time.
    let ticks_per_second = ticks_per_second().unwrap();

    // Get amount of time p has been scheduled in kernel mode and user mode at
    // this moment.
    let kernel_mode_time_before = p.stat.stime as i64 / ticks_per_second;
    let user_mode_time_before = p.stat.utime as i64 / ticks_per_second;

    // Let the sample time pass.
    //thread::sleep(time::Duration::from_millis(SAMPLE_TIME));

    // Get amount of time p has been scheduled in kernel mode and user mode
    // again.
    let kernel_mode_time_after = p.stat.stime as i64 / ticks_per_second;
    let user_mode_time_after = p.stat.utime as i64 / ticks_per_second;

    // Calculate total time in both modes and find their sum.
    let kernel_mode_time = kernel_mode_time_after - kernel_mode_time_before;
    let user_mode_time = user_mode_time_after - user_mode_time_before;
    let total_time = kernel_mode_time + user_mode_time;

    // Calculate total CPU usage over the sample time.
    let cpu_usage = (total_time / SAMPLE_TIME) as f32 * 100.0;

    // Send back the total CPU usage.
    return format_percent_usage(cpu_usage);
}

#[cfg(test)]
mod collector_tests {
    use sysinfo::{DiskExt, SystemExt};

    #[test]
    fn cpu_usage() {
        // Check this program's process ID.
        let this_process = procfs::process::Process::myself().unwrap();

        // Get the cpu usage of this process.
        let result = crate::collector::get_cpu_usage(&this_process);

        // Validate result.
        assert!(result >= 0.0);
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
        let result = crate::collector::get_disk_usage(&this_process, disk_space);

        // Validate result.
        assert!(result >= 0.0);
    }

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

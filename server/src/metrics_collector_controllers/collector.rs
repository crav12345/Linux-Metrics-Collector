use procfs::process::{FDTarget, Process};
use procfs::ticks_per_second;
use sysinfo::{DiskExt, NetworkExt, System, SystemExt};
use crate::metrics_collector_controllers::structs::Proc;
use crate::database::get_cpu_usage_by_pid;
use crate::{format_memory, format_percent_usage};
use log::{warn};

const SAMPLE_TIME: f32 = 15.0;

/*
This function calls for the different types of metrics to be collected from the operating system.
Metrics for each process are stored in a Proc struct. The struct is then added to a vector of Procs.
 */
pub fn collect_all_metrics(is_first_interval: bool) -> Vec<Proc> {
    let sys = System::new_all();
    let disks = sys.disks();
    let mut disk_space = 0;
    for disk in disks {
        disk_space += disk.available_space();
    }

    let mut net_data = 0;
    for (_interface_name, data) in sys.networks() {
        net_data += data.received() + data.transmitted();
    }

    // Collect Process Info
    let mut processes = Vec::new();
    for p in procfs::process::all_processes().unwrap() {
        // Use default constructor to create "null" process
        let mut new_process = Proc::default();

        let cpu_usage = collect_cpu_usage(&p, is_first_interval);
        let disk_usage = collect_disk_usage(&p, disk_space);
        let net_usage = collect_network_usage(&p, net_data);

        // get memory metrics from get_memory_usage
        let memory_info = collect_memory_usage(p);

        // set process object's fields to collected metrics
        new_process.set_pid(memory_info.0);
        new_process.set_name(memory_info.1);
        new_process.set_threads(memory_info.2);
        new_process.set_memory(memory_info.3);
        new_process.set_cpu_usage(cpu_usage.0);
        new_process.set_kernel_mode_time(cpu_usage.1);
        new_process.set_user_mode_time(cpu_usage.2);
        new_process.set_bytes_read(disk_usage.0);
        new_process.set_bytes_written(disk_usage.1);
        new_process.set_disk_usage(disk_usage.2);
        new_process.set_bytes_received(net_usage.0);
        new_process.set_bytes_transmitted(net_usage.1);
        new_process.set_net_usage(net_usage.2);

        processes.push(new_process);
    }

    return processes;
}

/*
This function takes in a process and returns a tuple of memory metrics
 */
pub fn collect_memory_usage(p: Process) -> (i32, String, i64, String) {
    let id = p.pid;
    let p_memory = p.stat.rss_bytes().unwrap();
    let p_name = p.stat.comm;
    let num_threads = p.stat.num_threads;
    let mem_usage = format_memory(p_memory);

    return (id, p_name, num_threads, mem_usage);
}

/*
This function takes in a process and returns a tuple of disk metrics
 */
// TODO: May need to do this over an interval because you get > 100% usage.
// TODO: Process not found bug causes crash.
pub fn collect_disk_usage(p: &Process, disk_space: u64) -> (
    String, String, String
) {
    let mut read = 0;
    let mut written = 0;

    // Checks the io file of a process.
    let io = p.io();
    let _io = match io {
        Ok(io_file) => {
            read = io_file.read_bytes;
            written = io_file.write_bytes;
        },
        Err(_error) => {
            warn!("Couldn't read io file for process {}", p.pid);
        },
    };

    // Calculate disk usage of this process as a percentage.
    let total_bytes = read as f32 + written as f32;
    let disk_usage = (total_bytes / disk_space as f32) * 100.0;

    // Kick the percentage of use back up.
    return (
        format_memory(read as i64),
        format_memory(written as i64),
        format_percent_usage(disk_usage)
    );
}

/*
This function takes in a process and returns a tuple of cpu metrics
 */
// TODO: Make tests for both first interval and all others.
// TODO: > 100% for some early intervals. May need to use is_first_interval.
pub fn collect_cpu_usage(p: &Process, is_first_interval: bool) -> (
    String, f32, f32
) {
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

    // String to be returned with CPU info.
    let mut cpu_usage_description: String = "LOADING".to_owned();

    // Only check usage if at least one sample interval has passed.
    if !is_first_interval {
        // Query database for this process' user mode and kernel mode time 15
        // seconds ago.
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
        let kernel_mode_time = kernel_mode_time_now - kernel_mode_time_before;
        let user_mode_time = user_mode_time_now - user_mode_time_before;
        let total_mode_time = kernel_mode_time + user_mode_time;

        // Calculate total CPU usage over the sample time.
        let cpu_usage = (total_mode_time / cpu_time_over_interval) as f32 * 100.0;

        // Update description to reflect usage as a percent if it is accurate.
        if cpu_usage <= 100.00 {
            cpu_usage_description = format_percent_usage(cpu_usage);
        }
    }

    return (cpu_usage_description, kernel_mode_time_now, user_mode_time_now);
}

/*
This function takes in a process and returns a tuple of network metrics
 */
pub fn collect_network_usage(p: &Process, net_data: u64) -> (
    String, String, String
) {
    let mut process_inode = 0;
    if let Ok(fds) = p.fd() {
        for fd in fds {
            if let FDTarget::Socket(inode) = fd.target {
                process_inode = inode;
            }
        }
    }

    let mut bytes_received: u32 = 0;
    let mut bytes_transmitted: u32 = 0;
    let mut total_usage: f32 = 0.0;
    let mut percent_usage: f32 = 0.0;

    // get the tcp table
    let tcp = procfs::net::tcp().unwrap();
    let tcp6 = procfs::net::tcp6().unwrap();
    for entry in tcp.into_iter().chain(tcp6) {
        if process_inode == entry.inode {
            bytes_received = entry.rx_queue;
            bytes_transmitted = entry.tx_queue;

            total_usage += bytes_received as f32 + bytes_transmitted as f32;

            if net_data > 0 {
                if total_usage > 0.0 {
                    percent_usage = (total_usage / net_data as f32) * 100.0;
                }
            }
        }
    }
    return (
        format_memory(bytes_received as i64),
        format_memory(bytes_transmitted as i64),
        format_percent_usage(percent_usage)
    );
}

#[cfg(test)]
mod collector_tests {
    use sysinfo::{DiskExt, SystemExt};
    use crate::collector::{collect_disk_usage, collect_cpu_usage, collect_memory_usage};

    #[test]
    fn cpu_usage() {
        use procfs::process::Process;

        // Check this program's process ID.
        let this_process = Process::myself().unwrap();

        // Get the cpu usage of this process.
        let result_vector = collect_cpu_usage(&this_process, false);

        if result_vector.0 != "LOADING" {
            // Get the amount of CPU usage and convert it to an f32 for comparison.
            let percent_usage = result_vector.0.replace("%", "");
            let result = percent_usage.parse::<f32>().unwrap();

            assert!(result >= 0.0);
        } else {
            // Validate result.
            assert_eq!(result_vector.0, "LOADING");
        }


    }

    #[test]
    fn disk_usage() {
        let sys = sysinfo::System::new_all();
        let disks = sys.disks();
        let mut disk_space = 0;
        for disk in disks {
            disk_space += disk.available_space();
        }

        // Check this program's process ID.
        let this_process = procfs::process::Process::myself().unwrap();

        // Get the cpu usage of this process.
        let percent_usage = collect_disk_usage(&this_process, disk_space).2
            .replace("%", "");

        // Convert the percent usage string to a float.
        let result = percent_usage.trim().parse::<f32>().unwrap();

        // Validate result.
        assert!(result >= 0.0);
    }

    #[test]
    fn test_collect_memory_usage() {
        // get process
        let p1 = procfs::process::all_processes().unwrap();
        let p2 = p1.first().unwrap();
        let p3 = p2.to_owned();
        let result = collect_memory_usage(p3);

        // Make sure that the returned metrics have values that make sense.
        assert!(result.0.is_positive());
        assert!(result.1.len() > 0);
        assert!(result.2 >= 0);
        assert!(result.3.len() > 2);
    }
}

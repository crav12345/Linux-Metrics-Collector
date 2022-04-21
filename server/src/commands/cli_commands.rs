use crate::format_percent_usage;
use crate::metrics_collector_controllers::database;
use crate::metrics_collector_controllers::collector;
use crate::metrics_collector_controllers::collector_utils;

pub fn display_database_info() {
    let process_info = database::get_current_metrics_from_db();

    println!("{0: <7} | {1: <43} | {2: <10} | {3: <7} | {4: <7}","PID", "NAME", "MEMORY", "CPU", "DISK");
    println!("_______________________________________________________________________________________");
    for p in process_info.unwrap() {
        println!("{0: <7} | {1: <43} | {2: <10} | {3: <7} | {4: <7}", p.proc_id, p.proc_name, p.proc_mem, p.proc_cpu, p.proc_disk_usage);
    }
}

pub fn display_cpu_info() {
    let process_info = database::get_current_metrics_from_db();

    println!("{0: <7}", "CPU");
    println!("-------");
    for p in process_info.unwrap() {
        println!("{0: <7}", p.proc_cpu);
    }
}

pub fn display_disk_info() {
    let process_info = database::get_current_metrics_from_db();

    println!("{0: <7}", "DISK");
    println!("-------");
    for p in process_info.unwrap() {
        println!("{0: <7}", p.proc_disk_usage);
    }
}

pub fn display_help_info() {
    println!("Marist Metrics Collector CLI Commands:");
    println!("     M -> Display all metrics information in the database");
    println!("     CPU -> Display CPU usage of all processes as a percent");
    println!("     Disk -> Display disk usage of all processes as a percent");
    println!("     Exit -> Quit the application");
}
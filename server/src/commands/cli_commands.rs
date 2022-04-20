use crate::metrics_collector_controllers::database;
use crate::metrics_collector_controllers::collector;

pub fn display_memory_info() {
    let process_info = database::get_current_metrics_from_db();

    println!("{0: <7} | {1: <43} | {2: <10} | {3: <4}","PID", "NAME", "MEMORY", "CPU");
    println!("________________________________________________________________________");
    for p in process_info.unwrap() {
        println!("{0: <7} | {1: <43} | {2: <10} | {3: <4}%", p.proc_id, p.proc_name, p.proc_mem, p.proc_cpu);
    }
}
use crate::metrics_collector_controllers::database;

pub fn display_memory_info() {
    let process_info = database::get_all_processes_from_db();

    println!("{: <7} {: <8} {}","PID", "NAME", "MEMORY");
    for p in process_info.unwrap() {
        println!("{: <7} {: <8} {}", p.proc_id, p.proc_name, p.proc_mem);
    }
}
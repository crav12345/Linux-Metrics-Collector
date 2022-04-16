use crate::metrics_collector_controllers::database;

pub fn display_memory_info() {
    let process_info = database::get_all_processes_from_db();

    println!("{0: <7} | {1: <48} | {2: <10}","PID", "NAME", "MEMORY");
    println!("________________________________________________________________________");
    for p in process_info.unwrap() {
        println!("{0: <7} | {1: <48} | {2: <10}", p.proc_id, p.proc_name, p.proc_mem);
    }
}
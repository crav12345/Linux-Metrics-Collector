pub mod collector_utils;

fn main() {
    collect_metrics();
}

fn collect_metrics() {
    use sysinfo::{ System, SystemExt};
    use collector_utils::Proc;

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Collect Process Info
    let mut processes = Vec::new();
    let mut rolling_mem: i64 = 0;

    for p in procfs::process::all_processes().unwrap() {
        let id = p.pid;
        let memory_use = p.stat.rss_bytes().unwrap();
        let p_name = p.stat.comm;
        let num_threads = p.stat.num_threads;

        let proc = Proc::new(id, &p_name, num_threads);
        processes.push(proc);

        rolling_mem += memory_use;

        println!("ID: {}, Name: {}, Num Threads: {}, Virt: {}", id, p_name, num_threads, memory_use);
    }

    collector_utils::print_processes(processes);

    println!("Rolling Mem: {}", rolling_mem);

}
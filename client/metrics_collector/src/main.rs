mod collector_utils;

struct Proces {
    proc_id: i32,
    proc_name: String,
    num_threads: i64,
    //mem_usage: String,
}

impl Proces {
    // Construct person
    fn new(id: i32, name: &str, threads: i64) -> Proces {
        Proces {
            proc_id: id,
            proc_name: name.to_string(),
            num_threads: threads,
            //mem_usage: mem_usage.to_string(),
        }
    }
}


fn main() {
    collect_metrics();
}

fn collect_metrics() {
    use sysinfo::{ System, SystemExt};
    use procfs::process::Process;

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    //let num_procs = sys.processes().len(); // works!

    // Collect Process Info
    let mut processes = Vec::new();
    let mut rolling_mem: i64 = 0;

    let me = Process::myself().unwrap();
    println!("PID: {:#?}", me);


    let mem = procfs::Meminfo::new().unwrap();
    println!("{:#?}", mem);
    println!("Active Memory: {:#?} kB", collector_utils::bytes_to_kb(mem.active));
    println!("Total  Memory: {:#?} kB", collector_utils::bytes_to_kb(mem.mem_total));


    for p in procfs::process::all_processes().unwrap() {
        let id = p.pid;
        let memory_use = p.stat.rss_bytes().unwrap();
        let p_name = p.stat.comm;
        let num_threads = p.stat.num_threads;

        let proc = Proces::new(id, &p_name, num_threads);
        processes.push(proc);


        rolling_mem += memory_use;


        println!("ID: {}, Name: {}, Num Threads: {}, Virt: {}", id, p_name, num_threads, memory_use);
    }

    println!("Rolling Mem: {}", rolling_mem);

    println!("{}", processes[0].proc_id);
    println!("{}", processes[0].proc_name);
    println!("{}", processes[0].num_threads);

}
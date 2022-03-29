
pub mod collector_utils;

fn main() {
    collect_metrics();
}

fn collect_metrics() {
    use sysinfo::{ System, SystemExt};
    use collector_utils::{ Device, Proc};

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Collect Memory Info (Most of this is uneccessary. Its just there bc I'm trying to
    // figure something out atm
    let mem = procfs::Meminfo::new().unwrap();
    let mem_total = collector_utils::format_memory(mem.mem_total as i64);
    let mem_active = collector_utils::format_memory(mem.active as i64);
    let mem_free = collector_utils::format_memory(mem.mem_free as i64);
    let mem_available = collector_utils::format_memory(mem.mem_available.unwrap() as i64);
    let mem_cached = collector_utils::format_memory(mem.cached as i64);
    let mem_swap_total = collector_utils::format_memory(mem.swap_total as i64);
    let mem_swap_free = collector_utils::format_memory(mem.swap_free as i64);
    let mem_swap_cached = collector_utils::format_memory(mem.swap_cached as i64);

    println!("Total Mem: {}", mem_total);
    println!("Active Mem: {}", mem_active);
    println!("Cached Mem: {}", mem_cached);
    println!("Free Mem: {}", mem_free);
    println!("Available Mem: {}", mem_available);
    println!("Total Mem Swap: {}", mem_swap_total);
    println!("Swap Mem Free: {}", mem_swap_free);
    println!("Swap Mem Cached: {}", mem_swap_cached);

    let device_info = Device::new(mem_total, mem_active);

    // Collect Process Info
    let mut processes = Vec::new();
    let mut rolling_mem: i64 = 0;

    for p in procfs::process::all_processes().unwrap() {
        let id = p.pid;
        let p_memory = p.stat.rss_bytes().unwrap();
        let p_name = p.stat.comm;
        let num_threads = p.stat.num_threads;
        let mem_usage = collector_utils::format_memory(p_memory);

        rolling_mem += p_memory;

        // Create new Proc struct and add it to our processes vector
        let proc = Proc::new(id, &p_name, num_threads, mem_usage);
        processes.push(proc);

        //println!("ID: {}, Name: {}, Num Threads: {}, Virt: {}", id, p_name, num_threads, memory_use);
    }
    collector_utils::print_device(device_info);
    collector_utils::print_processes(processes);

    println!("Rolling Mem: {}", rolling_mem);

}
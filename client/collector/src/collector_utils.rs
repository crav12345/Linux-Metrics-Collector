pub struct Proc {
    proc_id: i32,
    proc_name: String,
    num_threads: i64,
    // mem_usage: String,
}

impl Proc {
    // Construct person
    pub fn new(id: i32, name: &str, threads: i64) -> Proc {
        Proc {
            proc_id: id,
            proc_name: name.to_string(),
            num_threads: threads,
            //mem_usage: mem_usage.to_string(),
        }
    }
}

pub fn bytes_to_kb(bytes: u64) -> u64 {
    return bytes / 1000;
}


pub fn print_processes(processes: Vec<Proc>) {
    for p in processes {
        println!("PID: {}, Name: {}, Threads: {}", p.proc_id, p.proc_name, p.num_threads);
    }
}
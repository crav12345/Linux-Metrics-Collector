
pub struct Device {
    total_mem: String,
    active_mem: String,
}

impl Device {
    // construct device
    pub fn new(mem: String, active: String) -> Device {
        Device {
            total_mem: mem,
            active_mem: active,
        }
    }
}


pub struct Proc {
    proc_id: i32,
    proc_name: String,
    num_threads: i64,
    proc_mem: String,
}

impl Proc {
    // Construct process
    pub fn new(id: i32, name: &str, threads: i64, mem: String) -> Proc {
        Proc {
            proc_id: id,
            proc_name: name.to_string(),
            num_threads: threads,
            proc_mem: mem,
        }
    }
}

pub fn print_device(device: Device) {
    println!("Total Memory: {}, Memory Active {}", device.total_mem, device.active_mem);
}

pub fn print_processes(processes: Vec<Proc>) {
    for p in processes {
        println!("PID: {}, Name: {}, Threads: {}, Memory Use: {}", p.proc_id, p.proc_name,
                 p.num_threads, p.proc_mem);
    }
}

pub fn get_percent(num1: i64, num2: u64) -> String {
    let n1 = num1 as f64;
    let n2 = num2 as f64;
    let result = (n1 * 100.0) / n2;
    return format!("{:.3}%", result);
}

pub fn bytes_to_kb(bytes: u64) -> u64 {
    return bytes / 1000;
}

pub fn format_memory(bytes: i64) -> String {
    let bytesfloat= bytes as f64;
    if bytes >= 1000000000 {
        let answer = bytesfloat / 1000000000.0;
        return format!("{:.5} GB", answer);
    }
    else if bytes >= 1000000 {
        let answer = bytesfloat / 1000000.0;
        return format!("{:.5} MB", answer);
    }
    else if bytes >= 1000 {
        let answer = bytesfloat / 1000.0;
        return format!("{:.5} KB", answer);
    }
    return format!("{} B", bytes.to_string());
}
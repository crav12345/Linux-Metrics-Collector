use serde::Serialize;
use uuid::Uuid;

/*
// Not sure if we are still using this. Might not be needed
// Device Struct
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
 */

// Process Struct (Not too sure what line above it does. I'll look into it another time)
#[derive(Serialize)]
pub struct Proc {
    pub uuid: String,
    pub proc_id: i32,
    pub proc_name: String,
    pub num_threads: i64,
    pub proc_mem: String,
    pub proc_cpu: String,
    pub proc_disk_usage: String,
    pub proc_kernel_mode_time: f32,
    pub proc_user_mode_time: f32,
    pub proc_net_usage: String,
}

impl Default for Proc {
    fn default () -> Proc {
        Proc {
            uuid: Uuid::new_v4().to_string(),
            proc_id: 0,
            proc_name: "".to_owned(),
            num_threads: 0,
            proc_mem: "".to_owned(),
            proc_cpu: "".to_owned(),
            proc_disk_usage: "".to_owned(),
            proc_kernel_mode_time: 0.0,
            proc_user_mode_time: 0.0,
            proc_net_usage: "".to_owned(),
        }
    }
}

impl Proc {
    // Construct process
    pub fn new(uuid: String, id: i32, name: &str, threads: i64, mem: &str, cpu: &str, disk_usage: &str, kernel_mode_time: f32, user_mode_time: f32, net_usage: &str) -> Proc {
        Proc {
            uuid: Uuid::new_v4().to_string(),
            proc_id: id,
            proc_name: name.to_string(),
            num_threads: threads,
            proc_mem: mem.to_string(),
            proc_cpu: cpu.to_string(),
            proc_disk_usage: disk_usage.to_string(),
            proc_kernel_mode_time: kernel_mode_time,
            proc_user_mode_time: user_mode_time,
            proc_net_usage: net_usage.to_string(),
        }
    }

    pub fn set_pid(&mut self, pid: i32) {
        self.proc_id = pid;
    }

    pub fn set_pname(&mut self, pname: String) {
        self.proc_name = pname.to_string();
    }

    pub fn set_threads(&mut self, threads: i64) {
        self.num_threads = threads;
    }

    pub fn set_pmemory(&mut self, memory: String) {
        self.proc_mem = memory.to_string();
    }

    pub fn set_cpu_usage(&mut self, cpu_usage: String) { self.proc_cpu = cpu_usage; }

    pub fn set_disk_usage(&mut self, disk_usage: String) { self.proc_disk_usage = disk_usage; }

    pub fn set_kernel_mode_time(&mut self, kernel_mode_time: f32) { self.proc_kernel_mode_time = kernel_mode_time; }

    pub fn set_user_mode_time(&mut self, user_mode_time: f32) { self.proc_user_mode_time = user_mode_time; }

    pub fn set_net_usage(&mut self, net_usage: String) { self.proc_net_usage = net_usage; }
}

/*
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
*/

// Format methods.
pub fn format_memory(bytes: i64) -> String {
    let bytesfloat= bytes as f64;
    if bytes >= 1000000000 {
        let answer = bytesfloat / 1000000000.0;
        return format!("{:.2} GB", answer);
    }
    else if bytes >= 1000000 {
        let answer = bytesfloat / 1000000.0;
        return format!("{:.2} MB", answer);
    }
    else if bytes >= 1000 {
        let answer = bytesfloat / 1000.0;
        return format!("{:.2} KB", answer);
    }
    return format!("{:.2} B", bytes);
}

pub fn format_percent_usage(usage: f32) -> String {
    return format!("{:.2}%", usage);
}

#[cfg(test)]
mod utils_tests {
    use crate::format_percent_usage;

    // Test to make sure that the format_memory() function returns the expected values
    #[test]
    fn test_format_memory() -> Result<(), String> {
        assert_eq!(crate::format_memory(1234567), "1.23 MB");
        assert_eq!(crate::format_memory(1), "1 B");
        assert_eq!(crate::format_memory(9455000000), "9.46 GB");
        return Ok(());
    }

    #[test]
    fn test_format_percent_usage() -> Result<(), String> {
        assert_eq!(format_percent_usage(23.3664), "23.37%");
        assert_eq!(format_percent_usage(99.9999), "100.00%");
        assert_eq!(format_percent_usage(136.1354), "136.14%");
        return Ok(())
    }
}
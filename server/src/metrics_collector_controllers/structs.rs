/*
    This file essentially holds all of the models for the api. These structs are also used by the
    Metrics Collector to organize data.
 */

use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Process Struct
#[derive(Serialize)]
pub struct Proc {
    pub uuid: String,
    pub proc_id: i32,
    pub proc_name: String,
    pub num_threads: i64,
    pub proc_mem: String,
    pub proc_cpu: String,
    pub proc_bytes_read: String,
    pub proc_bytes_written: String,
    pub proc_disk_usage: String,
    pub proc_kernel_mode_time: f32,
    pub proc_user_mode_time: f32,
    pub proc_bytes_received: String,
    pub proc_bytes_transmitted: String,
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
            proc_bytes_read: "".to_owned(),
            proc_bytes_written: "".to_owned(),
            proc_disk_usage: "".to_owned(),
            proc_kernel_mode_time: 0.0,
            proc_user_mode_time: 0.0,
            proc_bytes_received: "".to_owned(),
            proc_bytes_transmitted: "".to_owned(),
            proc_net_usage: "".to_owned(),
        }
    }
}

impl Proc {
    // Construct process
    pub fn new(uuid: String, id: i32, name: &str, threads: i64, mem: &str, cpu: &str, bytes_read: &str, bytes_written: &str, disk_usage: &str, kernel_mode_time: f32, user_mode_time: f32, bytes_received: &str, bytes_transmitted: &str, net_usage: &str) -> Proc {
        Proc {
            uuid: Uuid::new_v4().to_string(),
            proc_id: id,
            proc_name: name.to_string(),
            num_threads: threads,
            proc_mem: mem.to_string(),
            proc_cpu: cpu.to_string(),
            proc_bytes_read: bytes_read.to_string(),
            proc_bytes_written: bytes_written.to_string(),
            proc_disk_usage: disk_usage.to_string(),
            proc_kernel_mode_time: kernel_mode_time,
            proc_user_mode_time: user_mode_time,
            proc_bytes_received: bytes_received.to_string(),
            proc_bytes_transmitted: bytes_transmitted.to_string(),
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

    pub fn set_cpu_usage(&mut self, cpu_usage: String) {
        self.proc_cpu = cpu_usage;
    }

    pub fn set_bytes_read(&mut self, bytes_read: String) { self.proc_bytes_read = bytes_read; }

    pub fn set_bytes_written(&mut self, bytes_written: String) { self.proc_bytes_written = bytes_written; }

    pub fn set_disk_usage(&mut self, disk_usage: String) {
        self.proc_disk_usage = disk_usage;
    }

    pub fn set_kernel_mode_time(&mut self, kernel_mode_time: f32) {
        self.proc_kernel_mode_time = kernel_mode_time;
    }

    pub fn set_user_mode_time(&mut self, user_mode_time: f32) {
        self.proc_user_mode_time = user_mode_time;
    }

    pub fn set_bytes_received(&mut self, bytes_received: String) { self.proc_bytes_received = bytes_received; }

    pub fn set_bytes_transmitted(&mut self, bytes_transmitted: String) { self.proc_bytes_transmitted = bytes_transmitted; }

    pub fn set_net_usage(&mut self, net_usage: String) {
        self.proc_net_usage = net_usage;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub proc_id: i32,
    pub proc_name: String,
    pub num_threads: i64,
    pub proc_mem: String
}

#[derive(Serialize, Deserialize)]
pub struct Disk {
    pub proc_id: i32,
    pub proc_name: String,
    pub proc_disk_usage: String
}

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
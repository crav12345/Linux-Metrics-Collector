
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

    // Test to ensure format_percent_usage() correctly formats f32 values.
    #[test]
    fn test_format_percent_usage() -> Result<(), String> {
        assert_eq!(format_percent_usage(0.233664), "23.36%");
    }
}
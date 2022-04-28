
pub fn format_memory(bytes: i64) -> String {
    let bytes_float = bytes as f64;
    if bytes >= 1000000000 {
        let answer = bytes_float / 1000000000.0;
        return format!("{:.2} GB", answer);
    }
    else if bytes >= 1000000 {
        let answer = bytes_float / 1000000.0;
        return format!("{:.2} MB", answer);
    }
    else if bytes >= 1000 {
        let answer = bytes_float / 1000.0;
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
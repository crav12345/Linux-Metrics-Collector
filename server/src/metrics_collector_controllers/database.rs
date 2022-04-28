use rusqlite::{Connection, Result, params};
use crate::metrics_collector_controllers::structs::{Proc, Memory, Disk};
use crate::collector::collect_all_metrics;

pub fn establish_connection() -> Result<()> {
    // Creates a database if it does not already exist
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path).unwrap();

    // Creates process table if it doesn't already exist
    conn.execute(
        "create table if not exists process (
             uuid text primary key,
             process_id integer,
             process_name text not null,
             num_threads integer not null,
             mem_usage text not null,
             cpu_usage text not null,
             bytes_read integer not null,
             bytes_written integer not null,
             disk_usage text not null,
             kernel_mode_time integer not null,
             user_mode_time integer not null,
             bytes_received integer not null,
             bytes_transmitted integer not null,
             net_usage text not null,
             date_created DATETIME not null DEFAULT(GETDATE())
         )",

        [],
    )?;

    // Creates current table for storing most recent info if it doesn't exist.
    conn.execute(
        "create table if not exists current (
             uuid text primary key,
             process_id integer,
             process_name text not null,
             num_threads integer not null,
             mem_usage text not null,
             cpu_usage text not null,
             bytes_read text not null,
             bytes_written text not null,
             disk_usage text not null,
             kernel_mode_time integer not null,
             user_mode_time integer not null,
             bytes_received text not null,
             bytes_transmitted text not null,
             net_usage text not null,
             date_created DATETIME not null DEFAULT(GETDATE())
         )",

        [],
    )?;
    Ok(())
}

pub fn store_data(processes: Vec<Proc>) -> Result<()> {
    // Creates a database if it does not already exist
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    // clear the 'current' table of old records
    conn.execute("DELETE FROM current",
                 [])?;

    // Go through each process
    for p in processes {
        // Stores the process in the 'process' table
        conn.execute(
            "INSERT INTO process (
                uuid, process_id, process_name, num_threads, mem_usage,
                cpu_usage, bytes_read, bytes_written, disk_usage,
                kernel_mode_time, user_mode_time, bytes_received,
                bytes_transmitted, net_usage, date_created
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                DATETIME()
            )",
            params![
                p.uuid, p.proc_id, p.proc_name, p.num_threads, p.proc_mem,
                p.proc_cpu, p.proc_bytes_read, p.proc_bytes_written,
                p.proc_disk_usage, p.proc_kernel_mode_time,
                p.proc_user_mode_time, p.proc_bytes_received,
                p.proc_bytes_transmitted, p.proc_net_usage
            ],
        )?;

        // Stores process in the 'current' table so that data can be retrieved.
        conn.execute(
            "INSERT INTO current (
                uuid, process_id, process_name, num_threads, mem_usage,
                cpu_usage, bytes_read, bytes_written, disk_usage,
                kernel_mode_time, user_mode_time, bytes_received,
                bytes_transmitted, net_usage, date_created
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                DATETIME()
             )",
            params![
                p.uuid, p.proc_id, p.proc_name, p.num_threads, p.proc_mem,
                p.proc_cpu, p.proc_bytes_read, p.proc_bytes_written,
                p.proc_disk_usage, p.proc_kernel_mode_time,
                p.proc_user_mode_time, p.proc_bytes_received,
                p.proc_bytes_transmitted, p.proc_net_usage
            ],
        )?;
    }
    Ok(())
}

pub fn update_data(is_first_interval: bool) {
    // Collect data on processes
    let processes: Vec<Proc> = collect_all_metrics(is_first_interval);

    // Send the vector of processes away to be stored in the database
    let _store_processes = store_data(processes);
    let _purge = purge_database();
}


pub fn get_current_metrics_from_db() -> Result<Vec<Proc>> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    let mut stmt = conn.prepare("SELECT * FROM current")?;

    let process_iter = stmt.query_map(params![], |row| {
        Ok(Proc {
            uuid: row.get(0)?,
            proc_id: row.get(1)?,
            proc_name:row.get(2)?,
            num_threads: row.get(3)?,
            proc_mem: row.get(4)?,
            proc_cpu: row.get(5)?,
            proc_bytes_read: row.get(6)?,
            proc_bytes_written: row.get(7)?,
            proc_disk_usage: row.get(8)?,
            proc_kernel_mode_time: row.get(9)?,
            proc_user_mode_time: row.get(10)?,
            proc_bytes_received: row.get(11)?,
            proc_bytes_transmitted: row.get(12)?,
            proc_net_usage: row.get(13)?
        })
    })?;

    let mut mem_data = Vec::new();
    for p in process_iter {
        mem_data.push(p.unwrap());
    }

    Ok(mem_data)
}

pub fn get_cpu_usage_by_pid(pid: i32) -> Result<Vec<f32>> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;
    let mut stmt = conn.prepare("SELECT * FROM current where process_id = ?")?;
    let mut rows = stmt.query(rusqlite::params![pid])?;

    let mut old_cpu_usage = Vec::new();
    while let Some(row) = rows.next()? {
        old_cpu_usage.push(row.get(9)?);
        old_cpu_usage.push(row.get(10)?);
    }

    Ok(old_cpu_usage)
}

pub fn purge_database() -> Result<()> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    conn.execute(
        "DELETE FROM process WHERE date_created < datetime('now', '-2 days')",
        [],
    )?;
    Ok(())
}


pub fn get_current_memory_info() -> Result<String> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    let mut stmt = conn
        .prepare(
            "SELECT process_id, process_name, num_threads, mem_usage \
            FROM current"
        )?;

    let mut mem_data: Vec<Memory> = Vec::new();

    let process_iter = stmt.query_map(params![], |row| {
        Ok(Memory {
            proc_id: row.get(0)?,
            proc_name:row.get(1)?,
            num_threads: row.get(2)?,
            proc_mem: row.get(3)?,
        })
    })?;

    // iterate through collected results to push each one to vector
    for p in process_iter {
        mem_data.push(p.unwrap());
    }

    // convert vector of results to JSON
    let json = serde_json::to_string_pretty(&mem_data).unwrap();

    Ok(json.to_string())
}

pub fn get_current_disk_info() -> Result<String> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    let mut stmt = conn
        .prepare("SELECT process_id, process_name, disk_usage FROM current")?;

    let mut disk_data: Vec<Disk> = Vec::new();

    let process_iter = stmt.query_map(params![], |row| {
        Ok(Disk {
            proc_id: row.get(0)?,
            proc_name:row.get(1)?,
            proc_disk_usage: row.get(2)?,
        })
    })?;

    // iterate through collected results to push each one to vector
    for p in process_iter {
        disk_data.push(p.unwrap());
    }

    // convert vector of results to JSON
    let json = serde_json::to_string_pretty(&disk_data).unwrap();

    Ok(json.to_string())
}

#[cfg(test)]
mod database_tests {
    use std::fs;

    #[test]
    fn test_establish_connection() {
        crate::database::establish_connection();
        assert!(fs::metadata(
            "src/metrics_collector_controllers/data.db"
        ).is_ok(), "db file does not exist");
    }

    #[test]
    fn test_store_data() {
        let example_process = crate::database::Proc::default();
        assert!(crate::database::store_data(vec![example_process]).is_ok());
    }

    #[test]
    fn test_get_current_metrics_from_db() {
        assert!(crate::database::get_current_metrics_from_db().is_ok());
    }
}
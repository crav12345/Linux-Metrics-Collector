use rusqlite::{Connection, Result, params};
use rusqlite::types::Value;
use crate::Proc;
use crate::collector::collect_all_metrics;

pub fn create_database() -> Result<()> {
    // Creates a database if it does not already exist
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    // Creates process table if it doesn't already exist
    conn.execute(
        "create table if not exists process (
             uuid text primary key,
             process_id integer,
             process_name text not null,
             num_threads integer not null,
             mem_usage text not null,
             cpu_usage text not null,
             disk_usage text not null,
             kernel_mode_time integer not null,
             user_mode_time integer not null,
             net_usage text not null,
             date_created DATETIME not null DEFAULT(GETDATE())
         )",

        [],
    )?;

    // Creates current table for storing the most recent info if it doesn't already exist
    conn.execute(
        "create table if not exists current (
             uuid text primary key,
             process_id integer,
             process_name text not null,
             num_threads integer not null,
             mem_usage text not null,
             cpu_usage text not null,
             disk_usage text not null,
             kernel_mode_time integer not null,
             user_mode_time integer not null,
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
            "INSERT INTO process (uuid, process_id, process_name, num_threads, mem_usage, cpu_usage, disk_usage, kernel_mode_time, user_mode_time, net_usage, date_created)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, DATETIME())",
            params![p.uuid, p.proc_id, p.proc_name, p.num_threads, p.proc_mem, p.proc_cpu, p.proc_disk_usage, p.proc_kernel_mode_time, p.proc_user_mode_time, p.proc_net_usage],
        )?;

        // Stores the process in the 'current' table so that current data can be easily retrieved
        conn.execute(
            "INSERT INTO current (uuid, process_id, process_name, num_threads, mem_usage, cpu_usage, disk_usage, kernel_mode_time, user_mode_time, net_usage, date_created)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, DATETIME())",
            params![p.uuid, p.proc_id, p.proc_name, p.num_threads, p.proc_mem, p.proc_cpu, p.proc_disk_usage, p.proc_kernel_mode_time, p.proc_user_mode_time, p.proc_net_usage],
        )?;
    }
    Ok(())
}

pub fn update_data(is_first_interval: bool) {
    // Collect data on processes
    let processes: Vec<Proc> = collect_all_metrics(is_first_interval);

    // Send the vector of processes away to be stored in the database
    let store_processes: Result<()> = store_data(processes);
    let purge: Result<()> = purge_database();
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
            proc_disk_usage: row.get(6)?,
            proc_kernel_mode_time: row.get(7)?,
            proc_user_mode_time: row.get(8)?,
            proc_net_usage: row.get(9)?
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
        old_cpu_usage.push(row.get(7)?);
        old_cpu_usage.push(row.get(8)?);
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


// NOTE: the convention for rust unit tests is that they live in the same file as the
//       code being tested

// avoid compiling unless 'cargo test' is entered
#[cfg(test)]
mod database_tests {
    use std::fs;

    // test to see if database file exists after running create_database()
    #[test]
    fn test_create_database() {
        crate::database::create_database();
        assert!(fs::metadata("src/metrics_collector_controllers/data.db").is_ok(),
                "db file does not exist");
    }

    // test that store_data() returns ok when trying to insert data into database
    #[test]
    fn test_store_data() {
        let example_process = crate::database::Proc::default();
        assert!(crate::database::store_data(vec![example_process]).is_ok());
    }

    // test that get_all_metrics_from_db() returns ok when attempting to pull all entries
    // from the database
    #[test]
    fn test_get_current_metrics_from_db() {
        assert!(crate::database::get_current_metrics_from_db().is_ok());
    }
}
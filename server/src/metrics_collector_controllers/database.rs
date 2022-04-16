use rusqlite::{Connection, Result, params};
use crate::Proc;
use crate::collector::collect_all_metrics;

pub fn open_database() -> Result<()> {
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

    for p in processes {
        // Creates process table if it doesn't already exist
        conn.execute(
            "INSERT INTO process (uuid, process_id, process_name, num_threads, mem_usage, date_created)
             VALUES (?1, ?2, ?3, ?4, ?5, DATETIME())",
            params![p.uuid, p.proc_id, p.proc_name, p.num_threads, p.proc_mem],
        )?;
    }
    Ok(())
}

pub fn update_data() {
    // Collect data on processes
    let processes: Vec<Proc> = collect_all_metrics();

    // Send the vector of processes away to be stored in the database
    let store_processes: Result<()> = store_data(processes);
}

pub fn get_all_processes_from_db() -> Result<Vec<Proc>> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    let mut stmt = conn.prepare("SELECT * FROM process")?;

    let process_iter = stmt.query_map(params![], |row| {
        Ok(Proc {
            uuid: row.get(0)?,
            proc_id: row.get(1)?,
            proc_name:row.get(2)?,
            num_threads: row.get(3)?,
            proc_mem: row.get(4)?,
        })
    })?;

    let mut mem_data = Vec::new();
    for p in process_iter {
        mem_data.push(p.unwrap());
    }

    Ok(mem_data)
}

/*
// TODO: PURGE DATABASE (DOES NOT WORK YET)
pub fn purge_database() -> Result<()> {
    let path = "src/metrics_collector_controllers/data.db";
    let conn = Connection::open(&path)?;

    conn.execute(
        "DELETE FROM process WHERE date_created < (DATETIME() - INTERVAL 2 MIN)",
        NO_PARAMS,
    )?;
    Ok(())
}*/

// NOTE: the convention for rust unit tests is that they live in the same file as the
//       code being tested

// avoid compiling unless 'cargo test' is entered
#[cfg(test)]
mod collector_tests {
    #[test]
    fn test_basic() {
        assert!(1 == 1);
    }

    #[test]
    fn test_equals() {
        assert_eq!(2, 1 + 1);
        assert_ne!(2, 1 + 2);
    }
}
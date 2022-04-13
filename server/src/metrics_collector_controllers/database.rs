use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;
use crate::Proc;

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

        NO_PARAMS,
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
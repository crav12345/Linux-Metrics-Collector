mod metrics_collector_controllers;

use metrics_collector_controllers::{collector, collector_utils, database};

use collector_utils::{Proc};
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn main() {
    // Open the database. Create it if it doesn't exist
    let result1: Result<()> = database::open_database();

    // Collect data on processes
    let processes: Vec<Proc> = collector::collect_all_metrics();

    // Send the vector of processes away to be stored in the database
    let result2: Result<()> = database::store_data(processes);

    /*
    let result3: Result<()> = database::purge_database();
    match result3 {
        Ok(sk) => {

        }
        Err(e) => {
            println!("{}", e);
        }
    }
     */

}


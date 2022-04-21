mod metrics_collector_controllers;
mod commands;

use metrics_collector_controllers::{collector, collector_utils, database};
use commands::cli_commands;
use sysinfo::{DiskExt, System, SystemExt};
use collector_utils::*;
use rusqlite::Result;
use std::time::Duration;
use std::io::Write;
use clokwerk::{Scheduler, TimeUnits};
use convert_case::{Case, Casing};
//use crate::collector::get_memory_usage;

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}

fn main() {
    // Open the database. Create it if it doesn't exist
    let establish_db: Result<()> = database::create_database();
    let fill_database = database::update_data();

    // Initialize scheduler thread
    let mut scheduler = Scheduler::new();

    // Have scheduler send current metrics to database every 15 seconds
    scheduler.every(15.seconds()).run(|| database::update_data());

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    println!("USE COMMAND 'HELP' FOR ALL CLI COMMANDS");
    loop {
        let input = prompt("MCC>  ");
        if input=="M" || input=="m" {
            cli_commands::display_database_info();
        } else if input.to_case(Case::Lower) == "cpu" {
            cli_commands::display_cpu_info();
        } else if input.to_case(Case::Lower) == "disk" {
            cli_commands::display_disk_info();
        } else if input.to_case(Case::Lower) == "help" {
            cli_commands::display_help_info();
        }
        else if input=="exit" {
            break;
        };
    }

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
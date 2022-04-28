mod metrics_collector_controllers;
mod commands;
use metrics_collector_controllers::{
    collector, collector_utils, database, handlers
};
use commands::cli_commands;
use collector_utils::*;
use std::time::Duration;
use std::io::Write;
use std::env;
use clokwerk::{Scheduler, TimeUnits};
use convert_case::{Case, Casing};
use actix_web::{
    App, HttpServer,
    middleware::Logger
};

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    let to_run = &args[1];

    // Open the database. Create it if it doesn't exist
    let _conn = database::establish_connection();
    database::update_data(true);

    // Initialize scheduler thread
    let mut scheduler = Scheduler::new();

    // Have scheduler send current metrics to database every 15 seconds
    scheduler.every(15.seconds()).run(|| database::update_data(false));

    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    if to_run == "cli" {
        println!("USE COMMAND 'HELP' FOR ALL CLI COMMANDS");
        loop {
            let input = prompt("MCC>  ");
            if input == "M" || input == "m" {
                cli_commands::display_database_info();
            } else if input.to_case(Case::Lower) == "cpu" {
                cli_commands::display_cpu_info();
            } else if input.to_case(Case::Lower) == "disk" {
                cli_commands::display_disk_info();
            } else if input.to_case(Case::Lower) == "net" {
                cli_commands::display_net_info();
            } else if input.to_case(Case::Lower) == "help" {
                cli_commands::display_help_info();
            } else if input == "exit" {
                break;
            };
        }
        return Ok(())
    }
    else {
        // Go to 'http://127.0.0.1:8080/' to test routes
        // Start http server
        HttpServer::new(move || {
            // Pass in default logger object
            Logger::default();
            // Create App Instance
            App::new()
                //.wrap(logger)
                .service(handlers::hello)
                .service(handlers::current_mem_info)
                .service(handlers::current_disk_info)
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }
}
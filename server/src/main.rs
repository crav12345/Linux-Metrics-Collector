// Declare modules.
mod controllers;
mod collector;

// Import methods from modules.
use controllers::task::{hello, echo};
use actix_web::{HttpServer, App, Responder, middleware::Logger};

// Macro to declare actix entry point.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set variables to inform actix-web whether to log or not.
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Create HTTP server struct.
    HttpServer::new(move || {
        // Pass in default logger object.
        let logger = Logger::default();

        // Create App Instance
        App::new()
            // pass logger in to give us logging
            .wrap(logger)
            // Use App::service for handlers using routing macros.
            .service(hello)
            .service(echo)
    })
        // Bind server struct to "127.0.0.1" port 8080.
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
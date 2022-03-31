// declare the existence of modules
mod api;

use api::task::{
    hello,
    echo,
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger, Responder};

// Go to 'http://127.0.0.1:8080/' to test route


// The macro in the line below lets actix know this is where to start running
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set logging variables that actix-web reads to determine whether to log or not
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Create HTTP Server Struct
    HttpServer::new(move || {
        // Pass in default logger object
        let logger = Logger::default();
        // Create App Instance
        App::new()
            // pass logger in to give us logging
            .wrap(logger)
            // Use App::service for handlers using routing macros.
            .service(hello)
            .service(echo)
    })
        // bind HTTP object to "127.0.0.1" port 80
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
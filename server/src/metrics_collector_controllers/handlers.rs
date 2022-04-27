use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize, Deserialize};
use crate::database::get_current_memory_info;
use crate::metrics_collector_controllers::structs::{Memory};

// handler functions for testing api
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/memory")]
pub async fn getCurrentMemInfo() -> impl Responder {
    let memory_info = get_current_memory_info();
    HttpResponse::Ok().body(memory_info.unwrap())
}

use actix_web::{HttpResponse, Responder, get};
use crate::database::{get_current_memory_info, get_current_disk_info};

// handler functions for testing api
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/memory")]
pub async fn current_mem_info() -> impl Responder {
    let memory_info = get_current_memory_info();
    HttpResponse::Ok().body(memory_info.unwrap())
}

#[get("/disk")]
pub async fn current_disk_info() -> impl Responder {
    let disk_info = get_current_disk_info();
    HttpResponse::Ok().body(disk_info.unwrap())
}

use actix_web::{HttpResponse, Responder, get, web, Resource};
use crate::database::{
    get_current_memory_info,
    get_current_disk_info,
    get_current_cpu_info,
    get_current_network_info,
    get_current_metrics_info
};

// handler functions for testing api
#[get("/api")]
pub async fn mmc_info() -> impl Responder {
    HttpResponse::Ok().body(
        "WELCOME TO THE MARIST METRICS COLLECTOR API! \n \n \
        Metrics can be found at the following URLs: \n \
        /metrics \n \
        /metrics/memory \n \
        /metrics/disk \n \
        /metrics/cpu \n \
        /metrics/network"
    )
}

#[get("/api/metrics")]
pub async fn current_metrics_info() -> impl Responder {
    let metrics_info = get_current_metrics_info();
    HttpResponse::Ok().body(metrics_info.unwrap())
}

#[get("/api/metrics/memory")]
pub async fn current_mem_info() -> impl Responder {
    let memory_info = get_current_memory_info();
    HttpResponse::Ok().body(memory_info.unwrap())
}

#[get("/api/metrics/disk")]
pub async fn current_disk_info() -> impl Responder {
    let disk_info = get_current_disk_info();
    HttpResponse::Ok().body(disk_info.unwrap())
}

#[get("/api/metrics/cpu")]
pub async fn current_cpu_info() -> impl Responder {
    let cpu_info = get_current_cpu_info();
    HttpResponse::Ok().body(cpu_info.unwrap())
}

#[get("/api/metrics/network")]
pub async fn current_network_info() -> impl Responder {
    let network_info = get_current_network_info();
    HttpResponse::Ok().body(network_info.unwrap())
}

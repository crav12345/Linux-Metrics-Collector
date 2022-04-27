use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize, Deserialize};
use crate::database::get_current_meminfo;
use crate::database::Memory;

// handler functions for testing api
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/memory")]
pub async fn getCurrentMemInfo() -> impl Responder {
    println!("HIT");
    //let result = get_current_meminfo();
   // for p in result.unwrap() {
    //    println!("{}", p.)
   // }
   // HttpResponse::Ok().body(result.unwrap())
    HttpResponse::Ok().body("TEST")
}

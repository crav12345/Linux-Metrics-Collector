/// This file defines api CRUD operations, routes, and controllers. CRUD
/// operations and routes are defined with macros which precede controller
/// methods. The controllers define business logic and HTTP responses to
/// requests from the client.

// Import dependencies.
use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode},
    Responder,
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
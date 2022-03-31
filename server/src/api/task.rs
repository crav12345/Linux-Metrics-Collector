// imports
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


// Request handler functions Below //

// Macros such as the on in the line below allow us to specify the method and the path that the
// handler should respond to.
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
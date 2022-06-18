use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder
};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap};

type LockedHashMap = Arc<Mutex<HashMap<Vec<u8>, bool>>>;

#[derive(Clone, Debug)]
pub struct HashAccess {
    pub value: LockedHashMap
}

#[get("/")]
async fn hello(data: web::Data<HashAccess>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}",data))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
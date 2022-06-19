use oob_poller_rust::tcp_server::TcpServer;
use clap::{Arg, Command};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::net::{SocketAddr};
use std::time::{ Instant};
use actix_web::{
    web, App, HttpServer
};
use oob_poller_rust::api::{hello, manual_hello, get_hash,check_key,remove_key, HashAccess};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Poller Started!");

    let app = Command::new("oob_poller_rust")
        .about("OOB Poller")
        .arg(Arg::new("ip_port").required(true))
        .get_matches();
    
    let ip_port = app.value_of("ip_port").unwrap();

    let hash: HashMap<String, Vec<(Instant, SocketAddr)>> = HashMap::new();
    let lock = Arc::new(Mutex::new(hash));
    let hashs_locked = Arc::clone(&lock);
    let web_hashs_locked = HashAccess {value: Arc::clone(&lock)};
    let tcp_server = TcpServer::new(String::from(ip_port), hashs_locked).unwrap();
    std::thread::spawn(move || {
        tcp_server.start();       
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(web_hashs_locked.clone()))
            .service(hello)
            .service(get_hash)
            .service(remove_key)
            .service(check_key)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await

   // thread::sleep(time::Duration::from_secs(500));
}
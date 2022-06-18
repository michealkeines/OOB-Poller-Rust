use oob_poller_rust::tcp_server::TcpServer;
use clap::{Arg, Command};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time;
use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder
};
use oob_poller_rust::api::{hello, manual_hello, echo, HashAccess};
type LockedHashMap = Arc<Mutex<HashMap<Vec<u8>, bool>>>;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Poller Started!");

    let app = Command::new("oob_poller_rust")
        .about("OOB Poller")
        .arg(Arg::new("ip_port").required(true))
        .get_matches();
    
    let ip_port = app.value_of("ip_port").unwrap();

    let mut hash: HashMap<Vec<u8>, bool> = HashMap::new();
    let mut lock = Arc::new(Mutex::new(hash));
    let hashs_locked = Arc::clone(&lock);
    let mut web_hashs_locked = HashAccess {value: Arc::clone(&lock)};
    let tcp_server = TcpServer::new(String::from(ip_port), hashs_locked).unwrap();
    std::thread::spawn(move || {
        tcp_server.start();       
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(web_hashs_locked.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await

   // thread::sleep(time::Duration::from_secs(500));
}
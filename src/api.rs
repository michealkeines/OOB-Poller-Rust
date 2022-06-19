use actix_web::{
    get,  web,  HttpResponse,  HttpRequest, Responder, Result
};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap};
use std::time::Instant;
use std::net::{SocketAddr, Ipv4Addr};

type LockedHashMap = Arc<Mutex<HashMap<String, Vec<(Instant, SocketAddr)>>>>;

#[derive(Clone, Debug)]
pub struct HashAccess {
    pub value: LockedHashMap
}

#[get("/")]
async fn hello(data: web::Data<HashAccess>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}",data))
}

#[get("/{hash_value}")]
async fn get_hash(path: web::Path<String>, data: web::Data<HashAccess>, req: HttpRequest) -> Result<String> {  
    //HttpResponse::Ok().body(req_body)
    let hash_value = path.into_inner();
    let remote_ip = req.peer_addr().unwrap();
    let ret = format!("Updated Hash: {}", hash_value);
    let mut lock = data.value.lock().unwrap();
    if let Some(list) = lock.get_mut(&hash_value) {
        list.push((Instant::now(), remote_ip));
    } else {
        lock.insert(hash_value, vec!((Instant::now(), remote_ip)));
    }   
    Ok(ret)
}

#[get("/check/{ip}/{hash_value}")]
async fn check_key(path: web::Path<(String,String)>, data: web::Data<HashAccess>) -> Result<String> {  
    let (ip, hash_value) = path.into_inner();
    
    let lock = data.value.lock().unwrap();
    if let Some(list) = lock.get(&hash_value) {
        for (instant, peer_addr) in list {
            let ipv4: Ipv4Addr = ip.parse().unwrap();
            if peer_addr.ip() == ipv4 {
                if Instant::now().duration_since(*instant) < std::time::Duration::from_secs(360) {
                    return Ok(format!("Is Vulnerable"));
                }
            }
        }
        Ok(format!("Not Vulnerable"))
    } else {
        Ok(format!("Key doesn't exist: {}",hash_value))
    }
}

#[get("/remove/{ip}/{hash_value}")]
async fn remove_key(path: web::Path<(String, String)>, data: web::Data<HashAccess>, req: HttpRequest) -> Result<String> {  
    let (ip, hash_value) = path.into_inner();
    let ret_ok = format!("Removed Hash: {}", hash_value);
    let mut lock = data.value.lock().unwrap();
    if let Some(list) = lock.get_mut(&hash_value) {
        if list.len() < 2 {
            for (_, peer_addr) in list{
                let ipv4: Ipv4Addr = ip.parse().unwrap();
                if peer_addr.ip() == ipv4 {
                    lock.remove(&hash_value).unwrap();
                    return Ok(ret_ok);
                }          
            }
            Ok(format!("Key doesn't exist: {}",hash_value))
        } else {
            let mut updated = vec![];
            for (instant, peer_addr) in list{
                let ipv4: Ipv4Addr = ip.parse().unwrap();
                if peer_addr.ip() != ipv4 {
                    updated.push((instant.clone(), peer_addr.clone()));
                }              
            }
            lock.remove(&hash_value).unwrap();
            lock.insert(hash_value, updated);
            return Ok(ret_ok);
        }
    } else {
        Ok(format!("Key doesn't exist: {}",hash_value))
    }
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
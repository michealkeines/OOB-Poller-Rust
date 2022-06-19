use std::net::{TcpListener, SocketAddr};
use std::io::{Read};
use std::collections::HashMap;
use std::thread;
use std::io;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use sha2::{Sha256, Digest};

type LockedHashMap = Arc<Mutex<HashMap<String, Vec<(Instant, SocketAddr)>>>>;

pub struct TcpServer {
    pub listener: TcpListener,
    pub hashes: LockedHashMap
}

impl TcpServer {
    pub fn new(ip_port: String, hashes: LockedHashMap) -> io::Result<TcpServer> {
        let listener = TcpListener::bind(ip_port)?;
        Ok(TcpServer {
            listener: listener,
            hashes: hashes
        })
    }
    pub fn start(mut self) -> io::Result<()> {
        let (tx, mut rx):(Sender<(Vec<u8>, SocketAddr)>, Receiver<(Vec<u8>,SocketAddr)>) = mpsc::channel();
        thread::spawn(move || {
            for stream in self.listener.incoming() {
                let mut stream = stream.unwrap();
                let peer = stream.peer_addr().unwrap();
                let mut buf = vec![];
                stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
                match stream.read_to_end(&mut buf) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Inside stream thread, read_to_end failed");}
                }
               // println!("{:?}",buf);
                if buf.len() != 0 {
                    tx.send((buf,peer)).unwrap();    
                }
            }
        });
        TcpServer::handle(&mut self.hashes,&mut rx);
        Ok(())
    }
    fn handle(hashes: &mut LockedHashMap, rx: &mut Receiver<(Vec<u8>, SocketAddr)>) {
        loop {
            if let Ok((buf, socket_addr)) = rx.recv_timeout(Duration::from_millis(150)) {
                let mut hasher = Sha256::new();
                hasher.update(&buf);
                let hash_value = format!("{:x}",hasher.finalize());
             //   println!("{}", v);

                let mut lock = hashes.lock().unwrap();
                if let Some(list) = lock.get_mut(&hash_value) {
                    list.push((Instant::now(), socket_addr));
                } else {
                    lock.insert(hash_value, vec!((Instant::now(), socket_addr)));
                }  
               // println!("{:?}",v);    
            }
            //
        }
    }
}
extern crate time;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

use time::{now_utc,strftime};

fn handle_client(mut stream: TcpStream) {
    let t = strftime("%Y-%m-%d %H:%M:%S %Z", &now_utc()).unwrap();
    println!("{}", t);
    let _ = stream.write(&t.as_bytes());
}

fn start_listening<A: ToSocketAddrs>(addr: A) {
    let listener = match TcpListener::bind(addr) {
        Ok(addr) => addr,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() {
    let ipv6 = thread::spawn(move||{
        start_listening("[::]:13");
    });
    let ipv4 = thread::spawn(move||{
        start_listening("0.0.0.0:13");
    });
    let _ = ipv6.join();
    let _ = ipv4.join();
}

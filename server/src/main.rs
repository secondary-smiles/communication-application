extern crate core;

mod messages;
mod security;
mod tcp;

use tcp::secure_loop::init;

use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4848").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let res = init::init(&mut stream);
        println!("{}", res);
        stream.write(res.to_string().as_bytes()).unwrap();
    }
}

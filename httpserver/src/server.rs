use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::str;

pub struct Server {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socker_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_litener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 200];
        }
    }
}

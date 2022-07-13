use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str
}

impl <'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        // 创建tcp监听
        let connect_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("running on {}", self.socket_addr);

        // 获取tcp接收的数据
        for stream in connect_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buf = [0; 200];

            stream.read(&mut read_buf).unwrap();
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}

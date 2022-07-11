use std::net::TcpStream;
use std::str;
use std::io::{Write, Read};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();


    println!("get response {:?}", str::from_utf8(&buffer).unwrap());
}

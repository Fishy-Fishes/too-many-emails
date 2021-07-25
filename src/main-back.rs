extern crate native_tls;

use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use bufstream::BufStream;

fn main() {
    let connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("imap.gmail.com:993").unwrap();
    let mut stream = BufStream::new(connector.connect("imap.gmail.com", stream).unwrap());
    let mut hello = String::new();
    stream.read_line(&mut hello).unwrap();
    println!("{}", hello);
    stream.write_all(b"0\r\n").unwrap();
    let mut hello = String::new();
    stream.read_line(&mut hello).unwrap();
    println!("{}", hello);
}
//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//
//     let response = "EHLO 149.167.149.26\r\n\r\n";
//
//     stream.write(response.as_bytes()).unwrap();
//     
//     stream.flush().unwrap();
//
//     stream.read(&mut buffer).unwrap();
//
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//
//     stream.write("AUTH LOGIN".as_bytes()).unwrap();
//
//     stream.read(&mut buffer).unwrap();
//
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//
//     stream.write("STARTTLS".as_bytes()).unwrap();
//
//     stream.read(&mut buffer).unwrap();
//
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }


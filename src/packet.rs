use std::net::TcpStream;

trait Packet {
    fn send_packet(mut stream: TcpStream);
    fn hey(mut strema: TcpStream);
}

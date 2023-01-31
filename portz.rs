use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Error, ErrorKind};

fn main() {
    let host = "localhost";
    let ports = vec![20, 21, 22, 23, 80, 443];
    for port in ports {
        match TcpStream::connect((host, port)) {
            Ok(_) => println!("Port {} is open", port),
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => println!("Port {} is closed", port),
                _ => println!("Error: {}", e),
            }
        }
    }
}

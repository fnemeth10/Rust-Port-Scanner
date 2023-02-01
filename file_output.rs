use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream};
use std::io::{ErrorKind};
    
fn main() {
    let host = "localhost";
    let max_port = 65535;
    let mut open_ports = 0;
    let mut closed_ports = 0;
    let mut output = String::new();
    output.push_str("Scanned 65535 ports\n");

    for port in 1..max_port {
        match TcpStream::connect((host, port)) {
            Ok(_) => {
                open_ports += 1;
                output.push_str(&format!("Port {} is open\n", port));
            },
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => {
                    closed_ports += 1;
                    output.push_str(&format!("Port {} is closed\n", port));
                },
                _ => {
                println!("Error: {}", e);
                output.push_str(&format!("Error: {}\n", e));
            }
        }
    }
}

output.push_str(&format!("{} open ports\n", open_ports));
output.push_str(&format!("{} closed ports\n", closed_ports));

let mut file = File::create("port_scan_output.txt").unwrap();
file.write_all(output.as_bytes()).unwrap();

}

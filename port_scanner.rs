use std::net::TcpStream;
use std::io::ErrorKind;

use std::env;

fn main() {
    let host = "localhost";
    let mut min_port = 0;
    let mut max_port = 1000;
    let mut open_ports = 0;
    let mut closed_ports = 0;

    let mut argv = env::args();
    _ = argv.next();
    while let Some(arg) = argv.next() {
        match &arg[..] {
            "-p" | "--ports" => {
                min_port = argv.next().unwrap_or_else(|| {
                    eprintln!("Missing min port");
                    std::process::exit(1);
                }).parse::<u16>().unwrap_or_else(|_| {
                    eprintln!("Min port must be a number");
                    std::process::exit(1);
                });
                max_port = argv.next().unwrap_or_else(|| {
                    eprintln!("Missing max port");
                    std::process::exit(1);
                }).parse::<u16>().unwrap_or_else(|_| {
                    eprintln!("Max port must be a number");
                    std::process::exit(1);
                });
            },
            _ => continue,
        }
    }

    println!("Scanning {} ports", max_port - min_port);

    for port in 1..max_port {
        match TcpStream::connect((host, port)) {
            Ok(_) => {
                open_ports += 1;
                println!("Port {} is open", port);
            },
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => closed_ports += 1,
                _ => println!("Error: {}", e),
            }
        }
    }

    println!("{} open ports", open_ports);
    println!("{} closed ports", closed_ports);
}

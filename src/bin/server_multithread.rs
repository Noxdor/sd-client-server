use std::net::{ SocketAddr, IpAddr, Ipv4Addr, TcpListener };
use std::io::{Read, Write};
use std::thread;

fn main() {
    // setup graceful shutdown handler
    ctrlc::set_handler(|| { 
        println!("Shutting down server."); 
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler.");

    // create ip address
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // create an ipv4 socket address
    let socket_addr = SocketAddr::new(localhost, 8000);

    // create a server socket
    let socket = TcpListener::bind(socket_addr).unwrap();

    println!("Started server");
    println!("Server address: {}\n", socket.local_addr().unwrap());

    // iterate over incoming tcp streams
    for stream in socket.incoming() {

        thread::spawn(|| {
            // get tcp stream if available
            let mut stream = stream.unwrap();

            // read stream into buffer
            let mut buffer = [0; 256];
            let number_bytes_read = stream.read(&mut buffer).unwrap();
            let stream_str = String::from_utf8_lossy(&buffer);

            println!("Received {} Bytes from client.\nEchoing: {}\n", number_bytes_read, stream_str);

            // write response into tcp stream for client to read
            let _bytes_written = stream.write(&buffer).unwrap();
        });
    }
}


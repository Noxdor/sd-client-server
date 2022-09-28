use std::net::{ SocketAddr, IpAddr, Ipv4Addr, TcpStream };
use std::io::{ Write, Read };
use std::str;


fn main() {
    // setup graceful shutdown handler
    ctrlc::set_handler(|| {
        println!("Shutting down client.");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler.");

    // get handlers to stdin and stdout
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // set ip address of server
    let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // set server socket address
    let server_addr = SocketAddr::new(ip_addr, 8000);


    println!("Started client\n");
    print!("Send to server: ");
    stdout.flush().unwrap();

    for line in stdin.lines() {
        // create a tcp stream to the server
        let mut stream = TcpStream::connect(server_addr).unwrap();

        // get input line from user
        let line = line.unwrap();

        // write line into tcp stream to server
        let _written_bytes = stream.write(line.as_bytes()).unwrap();

        // create buffer to read in server response
        let mut buffer = [0; 256];

        // reads in server response from tcp stream
        let _read_bytes = stream.read(&mut buffer).unwrap();

        // print out response from server
        println!("Received from server: {}\n", str::from_utf8(&buffer).unwrap());

        print!("Send to server: ");
        stdout.flush().unwrap();
    }
}

use std::iter;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::thread;
use std::time::{self, Duration, Instant};
use udt::*;

fn server() {
    let host = Ipv4Addr::UNSPECIFIED;
    let port = 1234;

    let listener = UdtSocket::new(SocketFamily::AFInet, SocketType::Stream).unwrap();

    listener.setsockopt(UdtOpts::UDP_RCVBUF, 5590000).unwrap();
    listener
        .bind(SocketAddr::V4(SocketAddrV4::new(host, port)))
        .unwrap();

    listener.listen(1).unwrap();

    let (socket, remote) = listener.accept().unwrap();
    println!("Received new connection from remote {:?}", remote);

    const BUFFER_SIZE: usize = 134217728;
    let mut buffer = vec![0; BUFFER_SIZE];

    let mut total = 0;

    let start = Instant::now();
    let mut last = Instant::now();

    loop {
        let read = socket.recv(buffer.as_mut(), BUFFER_SIZE).unwrap();
        total += read as u64;

        if last.elapsed() > Duration::from_secs(1) {
            last = Instant::now();
            println!("{}: {}", start.elapsed().as_secs_f64(), total);
        }
    }
}

fn client() {
    let host = Ipv4Addr::from_str("127.0.0.1").unwrap();
    let port = 1234;

    let socket = UdtSocket::new(SocketFamily::AFInet, SocketType::Stream).unwrap();

    socket.setsockopt(UdtOpts::UDP_RCVBUF, 5590000).unwrap();
    socket
        .connect(SocketAddr::V4(SocketAddrV4::new(host, port)))
        .unwrap();

    let buffer = (0..255).cycle().take(16384).collect::<Vec<_>>();

    loop {
        socket.send(buffer.as_ref()).unwrap();
    }
}

fn main() {
    thread::spawn(server);
    client();

    thread::sleep(Duration::from_secs(100));
}

use nix::sys::socket::*;
use nix::sys::socket::sockopt::ReuseAddr;
use nix::unistd::{close, read, write};
use std::os::fd::RawFd;

fn main() {
    println!("Hello, world!");

    // Access a raw file descriptor for a socket
    let fd = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None).unwrap();

    // Allow re-use of local address when re-binding
    setsockopt(fd, ReuseAddr, &true).expect("setsockopt()");

    // Bind and listen to address
    let addr = SockaddrIn::new(0, 0, 0, 0, 1234);
    bind(fd, &addr).expect("bind()");
    listen(fd, 4096).expect("listen()");

    // Loop over each incoming connection, accept them and do something
    loop {
        // Accept
        let conn_fd = match accept(fd) {
            Ok(fd) => fd,
            Err(e) => {
                println!("accept(): {}", e);
                continue;
            }
        };

        // Do something with the connection
        handle_conn(conn_fd);

        // Close the connection
        close(conn_fd).expect("close()");
    }
}

fn handle_conn(fd: RawFd) {
    let mut rbuf: [u8; 64] = [0; 64];
    let rsize = read(fd, &mut rbuf).expect("read()");

    println!("Client says: {}", String::from_utf8(rbuf.to_vec()).unwrap());

    write(fd, "world".as_bytes()).expect("write()");
}

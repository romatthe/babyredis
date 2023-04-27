use nix::sys::socket::*;
use nix::sys::socket::sockopt::ReuseAddr;
use nix::unistd::{close, read, write};
use std::os::fd::RawFd;

fn main() {
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

        // Handle the requests of a single connection until something goes wrong
        loop {
            // Handle the request, shut down connection on error
            if let Err(errno) = handle_request(conn_fd) {
                println!("Client error: {}", errno);
                break;
            }
        }

        // Close the connection
        close(conn_fd).expect("close()");
    }
}

fn handle_request(fd: RawFd) -> nix::Result<()> {
    let mut rbuf: [u8; 64] = [0; 64];
    let rsize = read(fd, &mut rbuf)?;

    println!("Client says: {}", String::from_utf8(rbuf.to_vec()).unwrap());

    write(fd, "world".as_bytes())?;

    Ok(())
}

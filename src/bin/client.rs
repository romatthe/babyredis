use nix::sys::socket::*;
use nix::unistd::{close, read, write};

fn main() {
    let fd = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None).unwrap();
    let addr = SockaddrIn::new(127, 0, 0, 1, 1234);

    // Connect to the server
    connect(fd, &addr).expect("connect()");

    // Write a string of UTF-8 bytes to the server
    write(fd, "hello".as_bytes()).expect("write()");

    // Read the response of UTF-8 bytes to a buffer
    let mut rbuf: [u8; 64] = [0; 64];
    let rsize = read(fd, &mut rbuf).expect("read()");

    println!("Server says: {}", String::from_utf8(rbuf.to_vec()).unwrap());

    // Close the connection, we're good
    close(fd).expect("close()");
}
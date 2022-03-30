use std::intrinsics::try;
use std::net::{TcpStream, ToSocketAddrs};

struct State {
    am_choking: bool,
    am_interested: bool,
    peer_choking: bool,
    peer_interested: bool
}

struct Peer {
    last_call: i32,
    has_handshake: bool,
    healthy: bool,
    read_buffer: Vec<u8>,
    stream: TcpStream,
    ip: str,
    port: u32,
    number_of_pieces: u32,
    bit_field: Vec<bool>,
    state: State
}

fn connect(host_and_port: Box<str>) {
    let mut addrs = host_and_port.to_socket_addrs().unwrap();
    if let Some(addr) =
}
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// Buffer space, sized to make the Packet structure fit in 2048 bytes (i.e. half
// a page on Linux)
const PACKET_BUF_SIZE: usize = 2000;

pub struct Packet {
    pub src: SocketAddr,
    pub len: usize,
    pub buf: [u8; PACKET_BUF_SIZE],
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            src: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
            len: 0,
            buf: [0u8; PACKET_BUF_SIZE],
        }
    }

    pub fn payload(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

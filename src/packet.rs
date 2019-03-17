use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::hash::{Hash, Hasher};

// Buffer space, sized to make the Packet structure fit in 2048 bytes (i.e. half
// a page on Linux)
const PACKET_BUF_SIZE: usize = 2000;

pub struct Packet {
    src: SocketAddr,
    len: usize,
    buf: [u8; PACKET_BUF_SIZE],
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            src: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
            len: 0,
            buf: [0u8; PACKET_BUF_SIZE],
        }
    }

    pub fn recv(&mut self, socket: &UdpSocket) -> std::io::Result<()> {
        let (n, src) = socket.recv_from(&mut self.buf)?;
        self.len = n;
        self.src = src;
        Ok(())
    }

    //pub fn payload(&self) -> &[u8] {
    //    &self.buf[..self.len]
    //}
}

impl Hash for Packet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
    }
}
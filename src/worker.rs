use crate::packet::Packet;
use std::net::UdpSocket;

pub struct Worker {
    socket: UdpSocket,
}

impl Worker {
    pub fn new(socket: UdpSocket) -> Worker {
        Worker { socket }
    }

    pub fn run(&self) {
        let mut pkt = Packet::new();
        loop {
            match self.recv(&mut pkt) {
                Ok(_) => self.on_recv(&mut pkt),
                Err(e) => error!("recv() failed: {}", e),
            }
        }
    }

    fn recv(&self, pkt: &mut Packet) -> std::io::Result<()> {
        let (n, src) = self.socket.recv_from(&mut pkt.buf)?;
        pkt.len = n;
        pkt.src = src;
        Ok(())
    }

    fn on_recv(&self, _pkt: &mut Packet) {}
}

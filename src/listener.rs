use crate::packet::Packet;
use nix::sys::socket::{self, sockopt::ReusePort};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::UdpSocket;
use std::os::unix::io::AsRawFd;
use std::sync::mpsc::SyncSender;

pub struct Listener {
    socket: UdpSocket,
    workers: Vec<SyncSender<Packet>>,
}

impl Listener {
    pub fn bind_socket(addr: &str) -> std::io::Result<UdpSocket> {
        let s = UdpSocket::bind(addr)?;
        socket::setsockopt(s.as_raw_fd(), ReusePort, &true).map_err(from_nix_error)?;
        Ok(s)
    }

    pub fn new(socket: UdpSocket, workers: Vec<SyncSender<Packet>>) -> std::io::Result<Listener> {
        let s = socket.try_clone()?;
        Ok(Listener { socket: s, workers })
    }

    pub fn listen(&self) -> std::io::Result<()> {
        loop {
            let mut pkt = Packet::new();
            pkt.recv(&self.socket)?;
            let worker_id = self.hash_packet(&pkt) as usize % self.workers.len();
            self.workers[worker_id].send(pkt);
        }
    }

    fn hash_packet(&self, pkt: &Packet) -> u64 {
        let mut s = DefaultHasher::new();
        pkt.hash(&mut s);
        s.finish()
    }
}

fn from_nix_error(err: ::nix::Error) -> std::io::Error {
    match err.as_errno() {
        Some(e) => std::io::Error::from_raw_os_error(e as i32),
        None => std::io::Error::new(std::io::ErrorKind::Other, "unknown nix error"),
    }
}

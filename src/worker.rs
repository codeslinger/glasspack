use std::sync::mpsc::Receiver;
use crate::packet::Packet;

pub struct Worker {
    rcv: Receiver<Packet>,
}

impl Worker {
    pub fn new(rcv: Receiver<Packet>) -> Worker {
        Worker { rcv }
    }

    pub fn run(&self) {
        loop {
            let pkt = self.rcv.recv();
        }
    }
}
mod listener;
mod packet;
mod worker;

use listener::Listener;
use std::sync::mpsc::sync_channel;
use std::thread;
use worker::Worker;

fn main() {
    let sock = Listener::bind_socket("0.0.0.0:2345").expect("could not bind listening socket");
    let mut workers = Vec::new();
    for _i in 0..1 {
        let (snd, rcv) = sync_channel(10);
        thread::spawn(move|| {
            let w = Worker::new(rcv);
            w.run();
        });
        workers.push(snd);
    }
    let l = Listener::new(sock, workers).expect("failed to create listener");
    l.listen().expect("listen() failed");
}

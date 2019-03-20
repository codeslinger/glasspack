#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod packet;
mod socket;
mod worker;

use clap::{App, Arg};
use env_logger;
use std::thread;

use socket::bind_worker_socket;
use worker::Worker;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = App::new("glasspack")
        .version(crate_version!())
        .arg(
            Arg::with_name("num_workers")
                .short("n")
                .long("num_workers")
                .value_name("INT")
                .help("Number of worker threads to run")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("listen_addr")
                .short("l")
                .long("listen_addr")
                .value_name("ADDRESS")
                .help("Address on which to listen")
                .takes_value(true)
                .default_value("0.0.0.0:2345"),
        )
        .get_matches();
    let n_workers = match matches.value_of("num_workers") {
        Some(n) => n.parse::<usize>().unwrap_or(1),
        None => 1,
    };
    let addr = matches.value_of("listen_addr").unwrap();

    let mut workers = Vec::new();
    for _i in 0..n_workers {
        let sock = bind_worker_socket(addr).expect("could not bind listening socket");
        let handle = thread::spawn(move || {
            let w = Worker::new(sock);
            w.run();
        });
        workers.push(handle);
    }

    info!(
        "started: listening on {} with {} worker(s)",
        addr,
        workers.len()
    );
    for handle in workers {
        handle.join().unwrap();
    }
}

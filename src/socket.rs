use nix::sys::socket::{
    bind, setsockopt, socket, sockopt, AddressFamily, InetAddr, SockAddr, SockFlag, SockProtocol,
    SockType,
};
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::os::unix::io::{FromRawFd, RawFd};
use std::str::FromStr;

const UDP_BUFFER_SIZE: usize = 16 * 1024 * 1024;

pub fn bind_worker_socket(addr: &str) -> io::Result<UdpSocket> {
    let sa: SocketAddr = FromStr::from_str(addr).expect("invalid address spec");
    let nix_addr = SockAddr::Inet(InetAddr::from_std(&sa));
    let fd = udp_socket(&sa)?;
    setsockopt(fd, sockopt::SndBuf, &UDP_BUFFER_SIZE).map_err(from_nix_error)?;
    setsockopt(fd, sockopt::RcvBuf, &UDP_BUFFER_SIZE).map_err(from_nix_error)?;
    setsockopt(fd, sockopt::ReuseAddr, &true).map_err(from_nix_error)?;
    setsockopt(fd, sockopt::ReusePort, &true).map_err(from_nix_error)?;
    bind(fd, &nix_addr).map_err(from_nix_error)?;
    let s = unsafe { UdpSocket::from_raw_fd(fd) };
    Ok(s)
}

fn udp_socket(sa: &SocketAddr) -> io::Result<RawFd> {
    let af = match sa {
        SocketAddr::V4(_) => AddressFamily::Inet,
        SocketAddr::V6(_) => AddressFamily::Inet6,
    };
    let fd = socket(af, SockType::Datagram, SockFlag::empty(), SockProtocol::Udp)
        .map_err(from_nix_error)?;
    Ok(fd)
}

fn from_nix_error(err: ::nix::Error) -> io::Error {
    match err.as_errno() {
        Some(e) => io::Error::from_raw_os_error(e as i32),
        None => io::Error::new(io::ErrorKind::Other, "unknown nix error"),
    }
}

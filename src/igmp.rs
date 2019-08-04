mod header;
mod socket;
mod types;

use libc::IPPROTO_IGMP;
use nix::sys::socket::sockopt::{IpMulticastLoop, IpMulticastTtl, RcvBuf, SndBuf};
use nix::sys::socket::{AddressFamily, SockFlag, SockType};
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;

use crate::igmp::socket::IgmpSocket;

pub fn init() -> IgmpSocket {
    let socket = crate::socket::socket(
        AddressFamily::Inet,
        SockType::Raw,
        SockFlag::empty(),
        IPPROTO_IGMP,
    )
    .map(|sock: RawFd| IgmpSocket::new(sock))
    .unwrap();
    socket
        .setsockopt(RcvBuf, &(5 * 1500))
        .expect("Unable to set rcvbuf");
    socket
        .setsockopt(SndBuf, &(5 * 1500))
        .expect("Unable to set sndbuf");
    socket
        .setsockopt(IpMulticastTtl, &1)
        .expect("unable to set mc ttl");
    socket
        .setsockopt(IpMulticastLoop, &false)
        .expect("unable to set mc loop to false");
    socket
        .join(Ipv4Addr::from([224, 0, 0, 22]))
        .expect("Unable to join IGMPv3 group 224.0.0.22");
    info!("IGMP Listener is initialized.");
    socket
}

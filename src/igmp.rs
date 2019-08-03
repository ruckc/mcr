use crate::fd;
use std::os::unix::io::RawFd;
use nix::sys::socket::{AddressFamily, SockFlag, SockType, SockProtocol, socket, recvfrom, SockAddr};
use nix::errno::Errno;

pub struct IgmpSocket {
    socket: RawFd,
    buf: [u8; 2000],
}

impl IgmpSocket {
    fn new(socket: RawFd) -> IgmpSocket {
        return IgmpSocket { socket, buf: [0; 2000] };
    }
}

impl fd::FD for IgmpSocket {
    fn fd(&self) -> RawFd {
        return self.socket;
    }

    fn handle(&mut self) {
        igmp_read(self);
    }
}

pub fn init() -> IgmpSocket {
    return socket(
        AddressFamily::Inet,
        SockType::Raw,
        SockFlag::empty(),
        SockProtocol::Udp,
    ).map(|sock: RawFd| IgmpSocket::new(sock)).unwrap();
}

fn igmp_read(socket: &mut IgmpSocket) {
    let mut len;
    let mut source: SockAddr;

    loop {
        let (l, sa) = recvfrom(socket.socket, &mut socket.buf).unwrap();

        source = sa;
        len = l;

        if Errno::last() == Errno::EINTR {
            continue;
        }

        break;
    }

    hexdump::hexdump(&socket.buf);

    let ippacket = crate::ip::IpPacket::parse(&socket.buf).unwrap();
    println! {"Received {} bytes from {} version={} (should be 4 or 6)", len, source, ippacket.ip.version};
}
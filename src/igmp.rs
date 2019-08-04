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

    let mut ip: crate::ip::IpHdr = Default::default();
    unsafe {
        std::ptr::copy_nonoverlapping(socket.buf.as_ptr(), &mut ip as *mut _ as *mut u8, len);
    }
    if ip.ihl() != 5 {

        return;
    }
    println! {"Received {} bytes from {} ver:{} ihl:{} from:{} to:{}", len, source, ip.ver(), ip.ihl(), ip.src(), ip.dst()};
}
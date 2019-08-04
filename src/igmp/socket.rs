use crate::fd::FD;
use crate::igmp::header::IgmpHeader;
use crate::igmp::types::IgmpType;
use crate::protocol::Protocol;
use nix::errno::Errno;
use nix::sys::socket::sockopt::{IpAddMembership, IpDropMembership};
use nix::sys::socket::{recvfrom, setsockopt, IpMembershipRequest, SetSockOpt, SockAddr};
use nix::Result;
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;

pub struct IgmpSocket {
    socket: RawFd,
    buf: [u8; 2000],
}

impl IgmpSocket {
    pub fn new(socket: RawFd) -> IgmpSocket {
        return IgmpSocket {
            socket,
            buf: [0; 2000],
        };
    }

    pub fn join(&self, group: Ipv4Addr) -> Result<()> {
        info!("Joining {} to mc group {}", self.socket, group);

        let request = IpMembershipRequest::new(nix::sys::socket::Ipv4Addr::from_std(&group), None);
        setsockopt(self.socket, IpAddMembership, &request)
    }

    pub fn leave(&self, group: Ipv4Addr) -> Result<()> {
        info!("Leaving mc group {} for {}", group, self.socket);

        let request = IpMembershipRequest::new(nix::sys::socket::Ipv4Addr::from_std(&group), None);
        setsockopt(self.socket, IpDropMembership, &request)
    }

    pub fn setsockopt<O: SetSockOpt>(&self, opt: O, val: &O::Val) -> Result<()> {
        setsockopt(self.socket, opt, val)
    }

    fn igmp_read(&mut self) {
        let mut len;
        let mut source: SockAddr;

        loop {
            let (l, sa) = recvfrom(self.socket, &mut self.buf).unwrap();

            source = sa;
            len = l;

            if Errno::last() == Errno::EINTR {
                continue;
            }

            break;
        }

        let packet = self.buf[0..len].to_vec();

        let mut ip: crate::ip::IpHeader = Default::default();
        unsafe {
            std::ptr::copy_nonoverlapping(packet.as_ptr(), &mut ip as *mut _ as *mut u8, len);
        }
        info!(
            "Received {} bytes from {} ver:{} ihl:{} from:{} to:{} proto:{:?}",
            len,
            source,
            ip.ver(),
            ip.ihl(),
            ip.src(),
            ip.dst(),
            ip.protocol()
        );

        let iphdrlen: usize = ip.ihl() * 4;
        let ipdata = packet[iphdrlen..len].to_vec();

        match ip.protocol() {
            Some(Protocol::IGMP) => {
                let mut igmp: IgmpHeader = Default::default();
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        ipdata.as_ptr(),
                        &mut igmp as *mut _ as *mut u8,
                        ipdata.len(),
                    );
                }
                match igmp.igmp_type() {
                    Some(IgmpType::IgmpMembershipQuery) => {}
                    Some(IgmpType::IgmpV1MembershipReport) => {}
                    Some(IgmpType::IgmpV2MembershipReport) => {}
                    Some(IgmpType::IgmpV3MembershipReport) => {}
                    Some(IgmpType::IgmpLeaveGroup) => {}
                    None => {
                        warn!("Unknown igmp message type {:x?}", igmp.itype);
                    }
                }
            }
            Some(v) => {
                warn!("Received packet for unhandled protocol type {:?}", v);
            }
            None => {
                warn!(
                    "Don't know what to do with packet of protocol {:?}",
                    ip.protocol()
                );
            }
        }
    }
}

impl FD for IgmpSocket {
    fn fd(&self) -> RawFd {
        return self.socket;
    }

    fn handle(&mut self) {
        self.igmp_read();
    }
}
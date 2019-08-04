use std::net::Ipv4Addr;
use crate::protocol::Protocol;

#[derive(Default)]
#[repr(C)]
pub struct IpHdr {
    pub ver_ihl: u8,
    pub tos: u8,
    pub len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub proto: u8,
    pub checksum: u16,
    pub saddr: u32,
    pub daddr: u32
}

pub struct IpPacket<B> {
    ip: IpHdr,
    body: B
}

impl IpHdr {
    pub fn ver(&self) -> u8 {
        return self.ver_ihl >> 4;
    }

    pub fn ihl(&self) -> u8 {
        return self.ver_ihl & 0x0F;
    }

    pub fn src(&self) -> Ipv4Addr {
        return Ipv4Addr::from(htonl(self.saddr));
    }

    pub fn dst(&self) -> Ipv4Addr {
        return Ipv4Addr::from(htonl(self.daddr));
    }

    pub fn protocol(&self) -> Protocol {
        return Protocol::from_u8(self.proto).unwrap();
    }
}

fn htonl(u: u32) -> u32 {
    return u.to_be();
}
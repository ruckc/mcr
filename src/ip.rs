use zerocopy::{AsBytes, FromBytes, LayoutVerified, ByteSlice, Unaligned};

#[derive(Default)]
#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct IpHdr {
    pub version: u32,
    pub header_len: u32,
    pub tos: u8,
    pub len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
}

pub struct IpPacket<B> {
    pub ip: LayoutVerified<B, IpHdr>,
    pub body: B,
}

impl<B: ByteSlice> IpPacket<B> {
    pub fn parse(bytes: B) -> Option<IpPacket<B>> {
        let (ip, body) = LayoutVerified::new_unaligned_from_prefix(bytes)?;
        Some(IpPacket { ip, body })
    }
}
#[derive(FromPrimitive, ToPrimitive)]
pub enum Protocol {
    IP,
    ICMP,
    IGMP,
    GGP,
    IPv4,
    ST,
    TCP,
    CBT,
    EGP,
    IGP,
    BBNRCC,
    NVPII,
    PUP,
    ARGUS,
    EMCON,
    XNET,
    CHAOS,
    UDP
}
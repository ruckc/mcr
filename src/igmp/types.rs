use crate::igmp::header::IgmpHeader;
use crate::ip::IpHeader;

#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum IgmpType {
    IgmpMembershipQuery = 0x11,
    IgmpV1MembershipReport = 0x12,
    IgmpV2MembershipReport = 0x16,
    IgmpV3MembershipReport = 0x22,
    IgmpLeaveGroup = 0x17,
}

struct IgmpMembershipQuery {
    ip: IpHeader,
    header: IgmpHeader,
}

struct IgmpV1MembershipReport {
    ip: IpHeader,
    header: IgmpHeader,
}

struct IgmpV2MembershipReport {
    ip: IpHeader,
    header: IgmpHeader,
}

struct IgmpV3MembershipReport {
    ip: IpHeader,
    header: IgmpHeader,
}

struct IgmpLeaveGroup {
    ip: IpHeader,
    header: IgmpHeader,
}

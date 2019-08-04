use crate::igmp::types::IgmpType;

use crate::num::FromPrimitive;

#[derive(Default)]
#[repr(C)]
pub struct IgmpHeader {
    pub itype: u8,
    code: u8,
    cksum: u16,
}

impl IgmpHeader {
    pub fn igmp_type(&self) -> Option<IgmpType> {
        IgmpType::from_u8(self.itype)
    }
}

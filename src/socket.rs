use nix::errno::Errno;
use nix::sys::socket::{AddressFamily, SockFlag, SockType};
use nix::Result;
use std::os::raw::c_int;
use std::os::unix::io::RawFd;

pub fn socket(
    domain: AddressFamily,
    ty: SockType,
    flags: SockFlag,
    protocol: i32,
) -> Result<RawFd> {
    let mut ty = ty as c_int;
    ty |= flags.bits();

    let res = unsafe { libc::socket(domain as c_int, ty, protocol) };

    return Errno::result(res);
}

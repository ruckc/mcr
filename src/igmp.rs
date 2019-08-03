pub struct IgmpSocket {
    pub socket: std::os::unix::io::RawFd
}

pub fn init() -> IgmpSocket {
    let rawsocket = nix::sys::socket::socket(
        nix::sys::socket::AddressFamily::Inet,
        nix::sys::socket::SockType::Raw,
        nix::sys::socket::SockFlag::empty(),
        None
    ).expect("Unable to create raw socket");
    let socket = IgmpSocket{socket: rawsocket};
    return socket;
}
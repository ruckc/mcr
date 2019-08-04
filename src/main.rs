mod fd;
mod igmp;
mod ip;
mod logger;
mod protocol;
mod socket;

extern crate libc;
#[macro_use]
extern crate log;
extern crate nix;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

use nix::sys::select::{select, FdSet};

use crate::fd::FD;
use crate::nix::Error::Sys;
use nix::errno::Errno;

fn main() {
    crate::logger::init();
    let mut igmp_socket = igmp::init();
    let mut sockets = [&mut igmp_socket as &mut dyn FD];

    loop {
        let mut fdset = FdSet::new();
        for s in sockets.iter() {
            fdset.insert(s.fd());
        }

        match select(None, Some(&mut fdset), None, None, None) {
            Err(Sys(Errno::EINTR)) => continue,
            Err(e) => error!("Error on select: {}", e),
            Ok(count) => {
                if count > 0 {
                    let ready_socks = sockets.iter_mut().filter(|s| fdset.contains(s.fd()));

                    for s in ready_socks {
                        s.handle();
                    }
                }
            }
        }
    }
}

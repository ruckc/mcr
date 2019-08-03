mod socket;
mod igmp;

pub extern crate nix;

use nix::sys::select::FdSet;
#[macro_use] extern crate log;
use crate::nix::Error::Sys;
use nix::errno::Errno;
use crate::socket::Socket;

fn main() {
    let mut igmp_socket = igmp::init();
    let sockets = [&mut igmp_socket as &dyn Socket];

    loop {
        let mut fdset = FdSet::new();
        for s in sockets.iter() {
            fdset.insert(s.socket());
        }

        match nix::sys::select::select(
            None,
            Some(&mut fdset),
            None,
            None,
            None,
        ) {
            Err(e) => match e {
                Sys(sysno) if sysno == Errno::EINTR => {
                    continue;
                }
                Sys(_) | _ => {
                    error!("Error on select: {}", e);
                }
            },
            Ok(count) => {
                if count > 0 {
                    for s in sockets.iter() {
                        if fdset.contains(s.socket()) {
                            s.handle();
                        }
                    }
                }
            }
        }
    }
}

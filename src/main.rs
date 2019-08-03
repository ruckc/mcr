mod fd;
mod igmp;

pub extern crate nix;

use nix::sys::select::{FdSet, select};

#[macro_use]
extern crate log;

use crate::nix::Error::Sys;
use nix::errno::Errno;
use crate::fd::FD;

fn main() {
    let mut igmp_socket = igmp::init();
    let mut sockets = [&mut igmp_socket as &mut dyn FD];

    loop {
        let mut fdset = FdSet::new();
        for s in sockets.iter() {
            fdset.insert(s.fd());
        }

        match select(
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
                    for s in sockets.iter_mut() {
                        if fdset.contains(s.fd()) {
                            s.handle();
                        }
                    }
                }
            }
        }
    }
}

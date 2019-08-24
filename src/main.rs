mod config;
mod fd;
mod igmp;
mod ip;
mod logger;
mod protocol;
mod socket;
mod vif;

extern crate clap;
extern crate clokwerk;
extern crate ini;
extern crate libc;
#[macro_use]
extern crate log;
extern crate nix;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate toml;

use crate::fd::FD;
use crate::nix::Error::Sys;
use nix::errno::Errno;
use nix::sys::select::{select, FdSet};

fn main() {
    let config = crate::config::init();
    crate::logger::init();

    // setup file descriptors
    let mut igmp_socket = igmp::init();
    let mut filedescriptors = [&mut igmp_socket as &mut dyn FD];

    let mut vifs = vif::init();

    // setup scheduled tasks
    let mut scheduler = clokwerk::Scheduler::new();

    loop {
        let mut fdset = FdSet::new();
        for s in filedescriptors.iter() {
            fdset.insert(s.fd());
        }

        // receive data from file descriptors (sockets) and call registered function
        match select(None, Some(&mut fdset), None, None, None) {
            Err(Sys(Errno::EINTR)) => continue,
            Err(e) => error!("Error on select: {}", e),
            Ok(count) => {
                if count > 0 {
                    let ready_socks = filedescriptors.iter_mut().filter(|s| fdset.contains(s.fd()));

                    for s in ready_socks {
                        s.handle();
                    }
                }
            }
        }

        // check scheduled tasks
        scheduler.run_pending();
    }
}

use std::os::unix::io::RawFd;

pub trait FD {
    fn fd(&self) -> RawFd;
    fn handle(&mut self);
}
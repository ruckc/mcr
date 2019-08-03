use std::os::unix::io::RawFd;

pub trait Socket {
    fn socket(&self) -> RawFd;
    fn handle(&mut self);
}
mod igmp;
pub extern crate nix;

fn main() {
    let igmp_socket = igmp::init();
    println!("Hello, world! {} ", igmp_socket.socket);
}

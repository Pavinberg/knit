mod core;
mod protocols;

use crate::core::packet::Packet;
use crate::protocols::ethernet::EthernetHeader;

fn main() {
    println!("Hello, world!");
	let mut packet = Packet::new(10);
	let eth_header = EthernetHeader::new(1, 2, 3);
	packet.add_header(eth_header);
	println!("{}", packet.size());
}

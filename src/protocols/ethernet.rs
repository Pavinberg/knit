use std::any::Any;
use crate::core::packet::Fragment;

#[allow(dead_code)]
/// The Ethernet ([IEEE 802.3](https://en.wikipedia.org/wiki/IEEE_802.3)) header.
pub struct EthernetHeader {
	preamble: u64,
	dst_addr: u64,
	src_addr: u64,
	length_type: u16,
}

impl EthernetHeader {
	/// Create a new `EthernetHeader`.
	pub fn new(src_addr: u64, dst_addr: u64, length_type: u16) -> EthernetHeader {
		EthernetHeader {
			preamble: 0,
			dst_addr,
			src_addr,
			length_type
		}
	}
}

impl Fragment for EthernetHeader {
	
	/// Get the number of bytes (8 + 14) of the `EthernetHeader`, preamble included.
	fn size(&self) -> usize {
		8 + 14
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn header_size() {
		let header = EthernetHeader::new(0, 0, 0);
		assert_eq!(header.size(), 8 + 14);
	}
}

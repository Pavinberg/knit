#![allow(dead_code)]

use std::any::Any;

///  `Fragment` is a series of bytes.
pub trait Fragment {
	/// Get the number of bytes of the `Fragment`.
    fn size(&self) -> usize;

	/// Used for dynamic downcast.
	fn as_any(&self) -> &dyn Any;
}

/// `Packet` is a collection of `Fragment`s and paylod.
/// Payload is just a number indicating the bytes of it for now.
pub struct Packet<'lp> {
    payload_size: usize,
    headers: Vec<Box<dyn Fragment + 'lp>>,
}

impl<'lp> Packet<'lp> {
	
	/// Create a new `Packet`.
    pub fn new(payload_size: usize) -> Self {
        Packet {
            payload_size,
            headers: vec![],
        }
    }

	/// Get the total size of the `Packet`, including headers and payload.
	pub fn size(&self) -> usize {
        self.headers.iter().fold(self.payload_size, |acc, h| acc + h.size())		
	}

	/// Add a header to the head of the `Packet`.
    pub fn add_header(&mut self, header: impl Fragment + 'lp) {
        self.headers.push(Box::new(header));
    }

	/// Peek a header of the `Packet` with an index.
	pub fn peek_header(&self, level: usize) -> &dyn Fragment {
        &*self.headers[level]
    }

	/// Peek the first header of the `Packet`.
	pub fn peek_first_header(&self) ->&dyn Fragment {
		self.peek_header(0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn raw_packet_size() {
		let sz1 = 0;
		let p1 = Packet::new(sz1);
		assert_eq!(p1.size(), sz1);

		let sz2 = 100;
		let p2 = Packet::new(sz2);
		assert_eq!(p2.size(), sz2);
	}

	struct TestHeader(u32);

	static TEST_HEADER_SIZE: usize = 4;

	impl Fragment for TestHeader {
		fn size(&self) -> usize {
			TEST_HEADER_SIZE
		}

		fn as_any(&self) -> &dyn Any {
			self
		}
		
	}

	#[test]
	fn add_header() {
		let field = 123;
		let header = TestHeader(field);
		let payload = 100;
		let mut p = Packet::new(payload);
		p.add_header(header);
		assert_eq!(p.size(), payload + TEST_HEADER_SIZE);
		if let Some(h) = p.headers[0].as_any().downcast_ref::<TestHeader>() {
			assert_eq!((*h).0, field);
		} else {
			assert!(false);
		}
		
	}
}

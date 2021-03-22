pub use wasm_std::Vec;
pub use wasm_std::String;
pub use wasm_std::types::*;
pub use crate::codec::Codec;


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	/// Invalid bool for provided input
	InvalidBool,
	/// Invalid u32 for provided input
	InvalidU32,
	/// Invalid u64 for provided input
	InvalidU64,
	/// The unexpected end of the stream
	UnexpectedEof,
	/// Invalid padding for fixed type
	InvalidPadding,
	/// Other error
	Other,
}



pub struct Stream<'a> {
	payload: &'a [u8],
	position: usize,
}

impl<'a> Stream<'a> {

	/// New stream for known payload
	pub fn new(raw: &'a [u8]) -> Self {
		Stream {
			payload: raw,
			position: 0,
		}
	}

	/// Pop next argument of known type
	pub fn pop<T: Codec>(&mut self) -> Result<T, Error> {
		if T::IS_DYNAMIC {
			T::decode(self)
		} else {
			let offset = u32::decode(self)?;
			let mut nested_stream = Stream::new(&self.payload[offset as usize..]);
			T::decode(&mut nested_stream)
		}
	}

	/// Current position for the stream
	pub fn position(&self) -> usize { self.position }

	/// Advance stream position for `amount` bytes
	pub fn advance(&mut self, amount: usize) -> Result<usize, Error> {
		if self.position + amount > self.payload.len() {
			return Err(Error::UnexpectedEof);
		}

		let old_position = self.position;
		self.position += amount;
		Ok(old_position)
	}

	/// Finish current advance, advancing stream to the next 32 byte step
	pub fn finish_advance(&mut self) {
		if self.position % 32 > 0 { self.position += 32 - (self.position % 32); }
	}

	/// Stream payload
	pub fn payload(&self) -> &[u8] {
		self.payload
	}

	/// Peek next byte in stream
	pub fn peek(&self) -> u8 {
		self.payload[self.position]
	}
}


pub struct Sink {
	capacity: usize,
	preamble: Vec<u8>,
	heap: Vec<u8>,
}

impl Sink {
	/// New sink with known capacity
	pub fn new(capacity: usize) -> Self {
		Sink {
			capacity: 32 * capacity,
			preamble: Vec::with_capacity(32 * capacity),
			heap: Vec::new(),
		}
	}

	fn top_ptr(&self) -> usize {
		self.preamble.capacity() + self.heap.len()
	}
	// new,push,finalize
	/// Consume `val` to the Sink
	pub fn push<T: Codec>(&mut self, val: T) {
		if T::IS_DYNAMIC {
			val.encode(self)
		} else {
			let mut nested_sink = Sink::new(1);
			val.encode(&mut nested_sink);
			let top_ptr = self.top_ptr() as u32;
			nested_sink.drain_to(&mut self.heap);
			self.push(top_ptr);
		}
	}

	/// Drain current Sink to the target vector
	pub fn drain_to(self, target: &mut Vec<u8>) {
		let preamble = self.preamble;
		let heap = self.heap;
		target.reserve(preamble.len() + heap.len());
		target.extend_from_slice(&preamble);
		target.extend_from_slice(&heap);
	}

	/// Consume current Sink to produce a vector with content.
	/// May panic if declared number of arguments does not match the resulting number of bytes should be produced.
	pub fn finalize_panicking(self) -> Vec<u8> {
		if self.preamble.len() != self.capacity { panic!("Underflow of pushed parameters {}/{}!", self.preamble.len(), self.capacity); }
		let mut result = self.preamble;
		let heap = self.heap;

		result.extend_from_slice(&heap);
		result
	}

	/// Mutable reference to the Sink preamble
	pub fn preamble_mut(&mut self) -> &mut Vec<u8> {
		&mut self.preamble
	}

	/// Mutable reference to the Sink heap
	pub fn heap_mut(&mut self) -> &mut Vec<u8> {
		&mut self.heap
	}
}

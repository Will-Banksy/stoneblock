use std::io::Read;

use tinyrand::{Rand, StdRand};

pub struct RandReader<'a> {
	rng: &'a mut StdRand
}

impl<'a> RandReader<'a> {
	pub fn new(rng: &'a mut StdRand) -> Self {
		RandReader { rng }
	}
}

impl<'a> Read for RandReader<'a> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		buf.chunks_exact_mut(4).for_each(|b| unsafe { *(b.as_mut_ptr() as *mut u32) = self.rng.next_u32() });
		let mut next = self.rng.next_u32();
		for b in buf.chunks_exact_mut(4).into_remainder() {
			*b = next as u8;
			next >>= 8;
		}

		Ok(buf.len())
	}
}

pub struct ZeroReader;

impl ZeroReader {
	pub fn new() -> Self {
		ZeroReader
	}
}

impl Read for ZeroReader {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		buf.fill(0);
		Ok(buf.len())
	}
}
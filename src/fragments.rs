use std::{fs::{self, File}, io::{self, BufReader, BufWriter, Read, Seek, SeekFrom}, rc::Rc};

use dyn_clone::DynClone;
use tinyrand::StdRand;

use crate::readers::{RandReader, ZeroReader};

pub trait Fragment: DynClone {
	/// Returns the length of the fragment, in bytes
	fn len(&self) -> u64;
	/// Returns a string representing the source of the fragment - This is usually a file path or string such as "random"
	fn source(&self) -> String;
	/// Returns the offset into the source, if that is relevant. Otherwise, returns 0
	fn source_offset(&self) -> u64;
	/// Writes the fragment's data to the given file. Should not keep any file data loaded, to avoid excessive memory consumption.
	/// Returns the number of bytes written (this should normally be the same as the length of the fragment)
	fn write(&mut self, output_file: &mut File) -> io::Result<u64>;
}

/// Fragments a file into `num_frags` fragments, where each fragment except the last has a length that is a multiple of `cluster_size`
pub fn fragment(path: String, num_frags: u32, block_size: u64) -> Vec<Rc<dyn Fragment>> {
	let file = File::open(path.clone()).unwrap();

	let file_len = file.metadata().unwrap().len();

	let fragment_size = {
		let ideal = file_len / num_frags as u64;
		if ideal <= block_size {
			panic!("The file cannot be split into {num_frags} fragments of (at least) {block_size} bytes - File is too small");
		}
		let block_size_rounded = (block_size..ideal).rev().find(|i| i % block_size == 0).expect(&format!("The file cannot be split into {num_frags} fragments of (at least) {block_size} bytes - File is too small"));
		block_size_rounded
	};

	let mut frags = Vec::new();

	for i in 0..num_frags as u64 {
		let start = i * fragment_size;
		let end = if i == num_frags as u64 - 1 {
			file_len
		} else {
			(i + 1) * fragment_size
		};

		frags.push(Rc::new(FileFragment {
			path: path.clone(),
			start,
			end
		}) as Rc<dyn Fragment>);
	}

	frags
}

#[derive(Clone)]
pub struct ZeroedFragment {
	len: u64
}

pub struct RandomFragment {
	rng: StdRand,
	len: u64
}

#[derive(Clone)]
pub struct FileFragment {
	path: String,
	start: u64,
	end: u64
}

impl ZeroedFragment {
	pub fn new(len: u64) -> Self {
		ZeroedFragment {
			len
		}
	}
}

impl Fragment for ZeroedFragment {
	fn len(&self) -> u64 {
		self.len
	}

	fn source(&self) -> String {
		"zeroes".to_string()
	}

	fn source_offset(&self) -> u64 {
		0
	}

	fn write(&mut self, output_file: &mut File) -> io::Result<u64> {
		let reader = BufReader::new(ZeroReader::new());
		let mut writer = BufWriter::new(output_file);

		let mut take = reader.take(self.len());
		io::copy(&mut take, &mut writer)
	}
}

impl RandomFragment {
	pub fn new(len: u64) -> Self {
		RandomFragment {
			len,
			rng: StdRand::default()
		}
	}
}

impl Clone for RandomFragment {
	fn clone(&self) -> Self {
		RandomFragment {
			len: self.len,
			rng: StdRand::default()
		}
	}
}

impl Fragment for RandomFragment {
	fn len(&self) -> u64 {
		self.len
	}

	fn source(&self) -> String {
		"random".to_string()
	}

	fn source_offset(&self) -> u64 {
		0
	}

	fn write(&mut self, output_file: &mut File) -> io::Result<u64> {
		let reader = BufReader::new(RandReader::new(&mut self.rng));
		let mut writer = BufWriter::new(output_file);

		let mut take = reader.take(self.len);
		io::copy(&mut take, &mut writer)
	}
}

impl Fragment for FileFragment {
	fn len(&self) -> u64 {
		self.end - self.start
	}

	fn source(&self) -> String { // TODO: This would perhaps be nicer if it returned the path relative to the corpus
		fs::canonicalize(self.path.clone()).unwrap().to_str().unwrap().to_string()
	}

	fn source_offset(&self) -> u64 {
		self.start
	}

	fn write(&mut self, output_file: &mut File) -> io::Result<u64> {
		let file = File::open(self.path.clone())?;

		let mut reader = BufReader::new(file);
		let mut writer = BufWriter::new(output_file);

		reader.seek(SeekFrom::Start(self.start))?;
		let mut take = reader.take(self.len()); // Reader.take and io::copy are so beautiful
		io::copy(&mut take, &mut writer)
	}
}
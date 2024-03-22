// TODO: Remove the bodgy stuff like the absolute mess of unwraps

use std::{collections::HashMap, env::args, fs::{self, File}, io::{self, Seek, SeekFrom, Write}, rc::Rc};

use serde::Deserialize;
use tinyrand::{Rand, StdRand};

#[derive(Debug, Deserialize)]
struct Config {
	block_size: u32,
	corpus: String,
	scenarios: HashMap<String, ConfigScenario>
}

#[derive(Debug, Deserialize)]
struct ConfigScenario {
	/// Relative to corpus
	path: String,
	files: Vec<ConfigFile>,
	layout: String
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
	path: String, // TODO: Make Option<String> and randomly select a file?
	fragments: u32
}

trait Fragment {
	/// Returns the length of the fragment, in bytes
	fn len(&self) -> usize;
	/// Writes the fragment's data to the given file. Should not keep any file data loaded, to avoid excessive memory consumption.
	/// Returns the number of bytes written (this should normally be the same as the length of the fragment)
	fn write(&self, output_file: &mut File) -> io::Result<usize>;
}

struct Scenario {
	path: String,
	layout: Vec<Rc<dyn Fragment>>
}

fn main() {
	let config_path = match args().nth(1) {
		Some(arg) => arg,
		None => panic!("Path to config file should be supplied as first argument")
	};

	let output_path = match args().nth(2) {
		Some(arg) => arg,
		None => panic!("Path to output file should be supplied as second argument")
	};

	let config: Config = toml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();

	println!("Config: {config:?}");

	todo!(); // TODO: Adapt the below code to use the config file to generate test images, and also write a report of where fragments are etc., like woodblock does (but we're gonna do it correctly)

	/*
	let mut output_file = File::create(&output_path).unwrap();

	let mut rand_data: Vec<u8> = vec![0; 1024];

	let mut rng = StdRand::default();

	for dir_entry in fs::read_dir(config.corpus).unwrap() {
		// Fill an amount of the output file with random data
		rand_data.chunks_exact_mut(4).for_each(|b| unsafe { *(b.as_mut_ptr() as *mut u32) = rng.next_u32() });
		let amt_rand = rng.next_lim_u32(1023);
		let written = output_file.write(&rand_data[0..amt_rand as usize]).unwrap();
		output_file.seek(SeekFrom::Current(written as i64)).unwrap();

		// Read the current directory entry (if it is a file) and appends that data to the output file
		// TODO: Introduce fragmentation, partially erase files, etc. for some more variation
		//       Also make sure the entire file is erased before writing to it - currently we're just
		//       overwriting the previous data, but if the previous data was longer than the current data,
		//       it will remain at the end of the file
		let dir_entry = dir_entry.unwrap();
		if dir_entry.metadata().unwrap().is_file() {
			let curr_fidx = output_file.seek(SeekFrom::Current(0)).unwrap();
			let file_data = fs::read(dir_entry.path()).unwrap();
			println!("Copying {} ({} bytes) into output file at idx {}", dir_entry.path().display(), file_data.len(), curr_fidx);
			let written = output_file.write(&file_data).unwrap();
			output_file.seek(SeekFrom::Current(written as i64)).unwrap();
		}
	}
	*/
}
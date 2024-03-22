mod fragments;
mod config;
mod readers;
mod layout;

// TODO: Remove the bodgy stuff like the absolute mess of unwraps

use std::{collections::{BTreeMap, HashMap}, env::args, fs::{self, File}, hash::Hash, io::{self, Seek, SeekFrom, Write}, rc::Rc};

use fragments::Fragment;
use serde::Deserialize;
use tinyrand::{Rand, StdRand};

use crate::{config::{Config, ConfigFile}, fragments::{fragment, RandomFragment}};

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

	let mut images: HashMap<String, Vec<Rc<dyn Fragment>>> = HashMap::new();

	for (_, scenario) in config.scenarios {
		let mut scenario_file_frags = Vec::new();

		for cfg_file in scenario.files {
			let mut file_frags = fragment(cfg_file.path, cfg_file.fragments, config.block_size);

			scenario_file_frags.append(&mut file_frags)
		}

		// TODO: Put fragments in layout order

		images.entry(scenario.path.clone()).and_modify(|e| {
			// e.append(&mut file_frags)
			todo!()
		}).or_insert_with(|| {
			// file_frags
			todo!()
		});
	}

	// TODO: Write images

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
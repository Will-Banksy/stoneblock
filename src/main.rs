mod fragments;
mod config;
mod readers;
mod layout;
mod utils;

// TODO: Remove the bodgy stuff like the absolute mess of unwraps, panics, etc.

use std::{collections::HashMap, env::args, fs::{self, File}, rc::Rc};

use fragments::Fragment;
use serde::Serialize;

use crate::{config::Config, fragments::{fragment, RandomFragment, ZeroedFragment}, layout::LayoutItem, utils::next_multiple_of};

struct Scenario {
	name: String,
	fragments: Vec<Rc<dyn Fragment>>
}

#[derive(Serialize)]
struct ImageMetadata {
	block_size: u64,
	corpus: String,
	scenarios: Vec<ScenarioMetadata>
}

#[derive(Serialize)]
struct ScenarioMetadata {
	name: String,
	fragments: Vec<FragmentMetadata>
}

#[derive(Serialize)]
struct FragmentMetadata { // TODO: Perhaps calculate hashes of fragments
	source: String, // TODO: Perhaps separate out source and source type, to avoid any ambiguity between a source type and a file path
	length: u64,
	source_offset: u64,
	image_offset: u64
}

fn main() {
	let config_path = match args().nth(1) {
		Some(arg) => arg,
		None => panic!("Path to config file should be supplied as first argument")
	};

	let config: Config = toml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();

	eprintln!("Config: {config:?}");

	let mut images: HashMap<String, Vec<Scenario>> = HashMap::new();
	let mut offsets: HashMap<String, u64> = HashMap::new();

	for (scenario_name, scenario) in config.scenarios {
		// A vec of vecs of fragments for each file index
		let mut scenario_file_frags = Vec::new();

		for cfg_file in scenario.files {
			let file_frags = fragment(utils::path_concat(&config.corpus, &cfg_file.path), cfg_file.fragments, config.block_size);

			scenario_file_frags.push(file_frags);
		}

		// Get the layout
		let layout = layout::parse_layout_str(&scenario.layout).unwrap();

		// Get the current offset in the file
		let mut curr_offset = if let Some(&offset) = offsets.get(&scenario.path) {
			offset
		} else {
			0
		};

		// Map the layout items to fragments, using the fill type to fill between files so that all files are allocated starting on cluster boundaries (padding is not automatically inserted)
		let fragments: Vec<Rc<dyn Fragment>> = layout.iter()
			.flat_map(|li| {
				match li {
					LayoutItem::Zeroed => {
						let frag_len = next_multiple_of(curr_offset, config.block_size) - curr_offset;
						curr_offset += frag_len;
						vec![
							Rc::new(ZeroedFragment::new(frag_len)) as Rc<dyn Fragment>
						]
					}
					LayoutItem::Random => {
						let frag_len = next_multiple_of(curr_offset, config.block_size) - curr_offset;
						curr_offset += frag_len;
						vec![
							Rc::new(RandomFragment::new(frag_len)) as Rc<dyn Fragment>
						]
					}
					LayoutItem::File { file_idx, fragment_idx } => {
						let file_frag = Rc::clone(
							&scenario_file_frags
								.get(*file_idx - 1).expect(&format!("Error: File of index {file_idx} does not exist ({scenario_name})"))
								.get(*fragment_idx - 1).expect(&format!("Error: Fragment of index {fragment_idx} in {file_idx} does not exist ({scenario_name})"))
						);
						if curr_offset % config.block_size != 0 {
							let filler_len = next_multiple_of(curr_offset, config.block_size) - curr_offset;
							let filler_frag = match scenario.filler.as_ref() {
								"Z" => Rc::new(ZeroedFragment::new(filler_len)) as Rc<dyn Fragment>,
								"R" => Rc::new(RandomFragment::new(filler_len)) as Rc<dyn Fragment>,
								value => panic!("Error: Invalid value for \"filler\" in config: {value} (should be \"Z\" or \"R\")")
							};

							curr_offset += filler_len + file_frag.len();
							vec![
								filler_frag,
								file_frag
							]
						} else {
							curr_offset += file_frag.len();
							vec![
								file_frag
							]
						}
					}
				}
			})
			.collect();

		images.entry(scenario.path.clone()).and_modify(|e| {
			e.push(Scenario {
				name: scenario_name.clone(),
				fragments: fragments.clone() // TODO: Optimise out this unnecessary copy
			});
		}).or_insert_with(|| {
			vec![
				Scenario {
					name: scenario_name,
					fragments
				}
			]
		});

		offsets.entry(scenario.path.clone()).and_modify(|off| {
			*off += curr_offset
		}).or_insert(curr_offset);
	}

	for (path, scenarios) in images {
		let mut output_file = File::create(&path).unwrap();
		let mut offset = 0;

		let mut image_meta = ImageMetadata {
			block_size: config.block_size,
			corpus: config.corpus.clone(),
			scenarios: Vec::new()
		};

		for scenario in scenarios {
			let mut frags_meta = Vec::new();

			for mut fragment in scenario.fragments {
				dyn_clone::rc_make_mut(&mut fragment).write(&mut output_file).unwrap();

				frags_meta.push(FragmentMetadata {
					source: fragment.source(),
					length: fragment.len(),
					source_offset: fragment.source_offset(),
					image_offset: offset
				});

				offset += fragment.len();
			}

			image_meta.scenarios.push(
				ScenarioMetadata {
					name: scenario.name,
					fragments: frags_meta
				}
			);
		}

		// Write out image metadata
		let metadata_path = format!("{path}.json");
		let mut buf = Vec::new();
		let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
		let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
		image_meta.serialize(&mut ser).unwrap();
		fs::write(metadata_path, {
			// &serde_json::to_string_pretty(&image_meta).unwrap()
			&buf
		}).unwrap();
	}

	eprintln!("Images written successfully");
}
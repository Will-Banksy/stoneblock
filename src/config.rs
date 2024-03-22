use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
	pub block_size: u64,
	pub corpus: String,
	pub scenarios: BTreeMap<String, ConfigScenario>
}

#[derive(Debug, Deserialize)]
pub struct ConfigScenario {
	/// Relative to corpus
	pub path: String,
	pub files: Vec<ConfigFile>,
	pub layout: String
}

#[derive(PartialEq, Eq, Hash, Debug, Deserialize)]
pub struct ConfigFile {
	pub path: String, // TODO: Make Option<String> and randomly select a file?
	pub fragments: u32
}
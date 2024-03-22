pub enum LayoutItem {
	Zeroed,
	Random,
	File {
		file_idx: usize,
		fragment_idx: usize,
	}
}

pub fn parse_layout_str(layout_str: &str) {
	let mut items: Vec<LayoutItem> = Vec::new();

	let item_strs = layout_str.split(",");

	for item_str in item_strs {
		todo!() // TODO: Parse each item_str into LayoutItems. Remember to trim the strings
	}
}
#[derive(Debug, PartialEq)]
pub enum LayoutItem {
	Zeroed,
	Random,
	File {
		file_idx: usize,
		fragment_idx: usize,
	}
}

pub fn parse_layout_str(layout_str: &str) -> Result<Vec<LayoutItem>, String> {
	let mut items: Vec<LayoutItem> = Vec::new();

	let item_strs = layout_str.split(",");

	for item_str in item_strs {
		let item_str = item_str.trim();

		if item_str == "Z" {
			items.push(LayoutItem::Zeroed);
		} else if item_str == "R" {
			items.push(LayoutItem::Random);
		} else {
			let idxs_strs: Vec<&str> = item_str.split(".").collect();
			if idxs_strs.len() == 2 {
				if let Ok(file_idx) = idxs_strs[0].trim().parse::<usize>() {
					if let Ok(fragment_idx) = idxs_strs[1].trim().parse::<usize>() {
						items.push(LayoutItem::File {
							file_idx,
							fragment_idx
						});

						continue;
					}
				}
			}

			return Err(format!("Error: \"{item_str}\" should be of the form <file_index>.<fragment_index> where file_index and fragment_index are integers"));
		}
	}

	Ok(items)
}

#[cfg(test)]
mod test {
    use super::{parse_layout_str, LayoutItem};

	#[test]
	fn test_parse_layout_str() {
		let test_str = "1.1, 	567.82, 2 .  4, R ,  Z";

		let expected = [
			LayoutItem::File { file_idx: 1, fragment_idx: 1 },
			LayoutItem::File { file_idx: 567, fragment_idx: 82 },
			LayoutItem::File { file_idx: 2, fragment_idx: 4 },
			LayoutItem::Random,
			LayoutItem::Zeroed
		];

		let result = parse_layout_str(test_str).unwrap();

		assert_eq!(result, expected);
	}
}
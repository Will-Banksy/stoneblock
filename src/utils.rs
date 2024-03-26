use std::path::PathBuf;

/// Calculates the next multiple of `multiple` of `num`. E.g. `next_multiple_of(7, 3) == 9`,
/// `next_multiple_of(9, 3) == 12`
pub fn next_multiple_of(num: u64, multiple: u64) -> u64 {
	((num / multiple) + 1) * multiple
}

/// Calculates the previous multiple of `multiple` of `num`. E.g. `prev_multiple_of(7, 3) == 6`,
/// `prev_multiple_of(9, 3) == 9`
pub fn prev_multiple_of(num: u64, multiple: u64) -> u64 {
	(num / multiple) * multiple
}

pub fn path_concat(dir: &str, file: &str) -> String {
	[
		dir,
		file
	].iter().collect::<PathBuf>().to_str().unwrap().to_owned()
}

#[cfg(test)]
mod test {
    use crate::utils::{next_multiple_of, prev_multiple_of};

	#[test]
	fn test_next_multiple_of() {
		assert_eq!(next_multiple_of(7, 3), 9);
		assert_eq!(next_multiple_of(9, 3), 12);
	}

	#[test]
	fn test_prev_multiple_of() {
		assert_eq!(prev_multiple_of(7, 3), 6);
		assert_eq!(prev_multiple_of(9, 3), 9);
	}
}
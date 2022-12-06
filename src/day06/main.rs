use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

fn has_unique_elements<T>(iter: T) -> bool
	where
		T: IntoIterator,
		T::Item: Eq + Hash,
{
	let mut uniq = HashSet::new();
	iter.into_iter().all(move |x| uniq.insert(x))
}

fn find_marker(stream: &str, marker_len: usize) -> usize {
	for i in 0..stream.len() {
		let marker = &stream[i..i + marker_len];
		let marker_valid = has_unique_elements(marker.chars());

		if marker_valid {
			return i + marker_len;
		}
	}

	return 0;
}

fn main() {
	println!("Day06");

	let input = fs::read_to_string("./input/day06.txt").unwrap();

	let part_1 = find_marker(&input, 4);
	println!("Part1: {:?}", part_1);

	let part_2 = find_marker(&input, 14);
	println!("Part2: {:?}", part_2);
}

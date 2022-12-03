use std::fs;

fn main() {
	println!("Day03");

	let input = fs::read_to_string("./input/day03.txt").unwrap();

	let part_1 = input.split("\n")
		.filter(|line| !line.is_empty())
		.map(|rucksack| rucksack.split_at((rucksack.len() as f32 * 0.5) as usize))
		.map(|(left, right)| {
			left.chars().find(|&x| right.chars().find(|&y| x == y).is_some()).unwrap()
		})
		.map(|a| if a.is_uppercase() { a as i32 - 65 + 27 } else { a as i32 - 97 + 1})
		.sum::<i32>();

	println!("Part1: {:?}", part_1);

	let part_2 = input.split("\n")
		.filter(|line| !line.is_empty())
		.collect::<Vec<_>>()
		.chunks(3)
		.map(|chunk| (chunk[0], chunk[1], chunk[2]))
		.map(|(a, b, c)| {
			a.chars().find(|&x| b.chars().find(|&y| x == y && c.chars().find(|&z| x == z).is_some()).is_some()).unwrap()
		})
		.map(|a| if a.is_uppercase() { a as i32 - 65 + 27 } else { a as i32 - 97 + 1})
		.sum::<i32>();

	println!("Part2: {:?}", part_2);
}

use std::fs;

fn main() {
	println!("Day01");

	let input = fs::read_to_string("./input/day01.txt").unwrap();

	let mut elf_stashes = input.split("\n\n")
		.map(|elf| elf.split("\n").filter_map(|elf_item| elf_item.parse::<i32>().ok()))
		.map(|elf_stash| elf_stash.reduce(|sum, elf_item| sum + elf_item).unwrap())
		.collect::<Vec<_>>();

	elf_stashes.sort_by(|a, b| b.cmp(a));

	let part_1 = elf_stashes[0];
	let part_2 = elf_stashes[0..3].iter().sum::<i32>();

	println!("Part1: {:?}", part_1);
	println!("Part2: {:?}", part_2);
}

use std::fs;

fn main() {
	println!("Day08");

	let input = fs::read_to_string("./input/day08.txt").unwrap();

	let processed = input.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| line.chars()
			.map(|char| char.to_digit(10).unwrap())
			.collect::<Vec<_>>()
		).collect::<Vec<_>>();
	let mut visibility: Vec<u32> = vec![];
	let mut scenic: Vec<usize> = vec![];

	for y in 0..processed.len() {
		for x in 0..processed[y].len() {
			let height = processed[y][x];

			let mut left = true;
			let mut right = true;
			let mut top = true;
			let mut bottom = true;
			let mut left_vis = 0;
			let mut right_vis = 0;
			let mut top_vis = 0;
			let mut bottom_vis = 0;

			for to_x in (0..x).rev() {
				left_vis = x - to_x;

				if processed[y][to_x] >= height {
					left = false;
					break;
				}
			}

			for to_y in (0..y).rev() {
				top_vis = y - to_y;

				if processed[to_y][x] >= height {
					top = false;
					break;
				}
			}

			for from_x in x + 1..processed[y].len() {
				right_vis = from_x - x;

				if processed[y][from_x] >= height {
					right = false;
					break;
				}
			}

			for from_y in y + 1..processed.len() {
				bottom_vis = from_y - y;

				if processed[from_y][x] >= height {
					bottom = false;
					break;
				}
			}

			visibility.push(if left || right || top || bottom { 1 } else { 0 });
			scenic.push(left_vis * right_vis * top_vis * bottom_vis);
		}
	}

	let part_1 = visibility.iter().sum::<u32>();
	println!("Part1: {:?}", part_1);

	scenic.sort();

	let part_2 = scenic.last().unwrap();
	println!("Part2: {:?}", part_2);
}

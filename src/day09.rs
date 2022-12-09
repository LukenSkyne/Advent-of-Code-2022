use std::fs;

type Point<T> = (T, T);

fn main() {
	println!("Day09");

	let input = fs::read_to_string("./input/day09.txt").unwrap();

	let start_pos: Point<i32> = (0, 0);
	let mut visited_part_1 = vec![start_pos];
	let mut visited_part_2 = vec![start_pos];
	let mut knots = vec![start_pos; 10];

	input.split("\n")
		.filter(|line| !line.is_empty())
		.for_each(|line| {
			let (direction, count_str) = line.split_once(" ").unwrap();
			let count: i32 = count_str.parse().unwrap();

			for _ in 0..count {
				match direction {
					"L" => { knots[0].1 -= 1; }
					"R" => { knots[0].1 += 1; }
					"U" => { knots[0].0 -= 1; }
					"D" => { knots[0].0 += 1; }
					_ => {}
				}

				for i in 1..knots.len() {
					let delta_y = knots[i-1].0 - knots[i].0;
					let delta_x = knots[i-1].1 - knots[i].1;

					if delta_y.abs() > 1 || delta_x.abs() > 1 {
						knots[i].0 += delta_y.signum();
						knots[i].1 += delta_x.signum();
					}

					if i == 1 && !visited_part_1.contains(&knots[i]) {
						visited_part_1.push(knots[i]);
					}

					if i == knots.len() - 1 && !visited_part_2.contains(&knots[i]) {
						visited_part_2.push(knots[i]);
					}
				}
			}
		});

	let part_1 = visited_part_1.len();
	println!("Part1: {:?}", part_1);

	let part_2 = visited_part_2.len();
	println!("Part2: {:?}", part_2);
}

use std::fs;

fn check_signal_strength(cycle_count: i32, register: i32, signal_strength_sum: &mut i32) {
	let signal_strength = cycle_count * register;

	if cycle_count % 40 == 20 {
		*signal_strength_sum += signal_strength;
	}
}

fn crt_tick(cycle_count: i32, sprite_pos: i32, crt: &mut Vec<Vec<char>>) {
	let col = (cycle_count - 1) % 40;
	let row = (cycle_count - 1) / 40;

	if ((sprite_pos-1)..(sprite_pos+2)).contains(&col) {
		crt[row as usize][col as usize] = '#';
	}
}

fn print_crt(crt: &Vec<Vec<char>>) {
	for y in 0..crt.len() {
		for x in 0..crt[y].len() {
			print!("{}", crt[y][x]);
		}

		println!();
	}
}

fn main() {
	println!("Day10");

	let input = fs::read_to_string("./input/day10.txt").unwrap();

	let mut register = 1;
	let mut cycle_count = 1;
	let mut sum_signal_strength = 0;
	let mut crt = vec![vec!['.'; 40]; 6];

	input.split("\n")
		.filter(|line| !line.is_empty())
		.for_each(|line| {
			check_signal_strength(cycle_count, register, &mut sum_signal_strength);
			crt_tick(cycle_count, register, &mut crt);
			cycle_count += 1;

			if line == "noop" {
				return
			}

			let (_, amt_str) = line.split_once(" ").unwrap();
			let amt: i32 = amt_str.parse().unwrap();

			check_signal_strength(cycle_count, register, &mut sum_signal_strength);
			crt_tick(cycle_count, register, &mut crt);
			cycle_count += 1;
			register += amt;
		});

	println!("Part1: {:?}", sum_signal_strength);
	println!("Part2:");
	print_crt(&crt);
}

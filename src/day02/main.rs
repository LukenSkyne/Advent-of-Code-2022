use std::fs;

enum Tool {
	Rock,
	Paper,
	Scissors,
}

fn is(a: i32, b: Tool) -> bool {
	return a == b as i32;
}

fn main() {
    println!("Day02");

	let input = fs::read_to_string("./input/day02.txt").unwrap();
	
	let part_1 = input.split("\n")
		.filter(|a| !a.is_empty())
		.map(|round| round.split_once(" ").unwrap())
		.map(|(opp, you)| (
			opp.as_bytes()[0] as i32 - 'A' as i32,
			you.as_bytes()[0] as i32 - 'X' as i32
		))
		.map(|(opp, you)| {
			let mut score = you + 1;

			if opp == you {
				score += 3;
			} else if
				(is(you, Tool::Rock) && is(opp, Tool::Scissors)) ||
				(is(you, Tool::Paper) && is(opp, Tool::Rock)) ||
				(is(you, Tool::Scissors) && is(opp, Tool::Paper)) {
				score += 6;
			}

			score
		})
		.sum::<i32>();

	let part_2 = input.split("\n")
		.filter(|a| !a.is_empty())
		.map(|round| round.split_once(" ").unwrap())
		.map(|(opp, you)| (
			opp.as_bytes()[0] as i32 - 'A' as i32,
			you.as_bytes()[0] as i32 - 'X' as i32
		))
		.map(|(opp, mut you)| {
			match you {
				0 => {
					// lose
					if is(opp, Tool::Rock) {
						you = Tool::Scissors as i32
					} else if is(opp, Tool::Paper) {
						you = Tool::Rock as i32
					} else if is(opp, Tool::Scissors) {
						you = Tool::Paper as i32
					}
				},
				1 => {
					// draw
					you = opp
				},
				2 => {
					// win
					if is(opp, Tool::Rock) {
						you = Tool::Paper as i32
					} else if is(opp, Tool::Paper) {
						you = Tool::Scissors as i32
					} else if is(opp, Tool::Scissors) {
						you = Tool::Rock as i32
					}
				},
				_ => {},
			}

			(opp, you)
		})
		.map(|(opp, you)| {
			let mut score = you + 1;

			if opp == you {
				score += 3;
			} else if
				(is(you, Tool::Rock) && is(opp, Tool::Scissors)) ||
				(is(you, Tool::Paper) && is(opp, Tool::Rock)) ||
				(is(you, Tool::Scissors) && is(opp, Tool::Paper)) {
				score += 6;
			}

			score
		})
		.sum::<i32>();

	println!("Part1: {:?}", part_1);
	println!("Part2: {:?}", part_2);
}

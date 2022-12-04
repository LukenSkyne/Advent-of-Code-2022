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
	let processed = input.split("\n")
		.filter(|a| !a.is_empty())
		.map(|round| round.split_once(" ").unwrap())
		.map(|(opp, you)| (
			opp.as_bytes()[0] as i32 - 'A' as i32,
			you.as_bytes()[0] as i32 - 'X' as i32
		)).collect::<Vec<_>>();

	let calc_score = |(opp, you)| {
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
	};

	let part_1 = processed.iter()
		.map(|&game| calc_score(game))
		.sum::<i32>();

	println!("Part1: {:?}", part_1);

	let part_2 = processed.iter()
		.map(|&(opp, mut you)| {
			if you == 1 {
				you = opp;
			} else if is(opp, Tool::Rock) {
				you = if you == 2 { Tool::Paper as i32 } else { Tool::Scissors as i32 };
			} else if is(opp, Tool::Paper) {
				you = if you == 2 { Tool::Scissors as i32 } else { Tool::Rock as i32 };
			} else if is(opp, Tool::Scissors) {
				you = if you == 2 { Tool::Rock as i32 } else { Tool::Paper as i32 };
			}

			(opp, you)
		})
		.map(|game| calc_score(game))
		.sum::<i32>();

	println!("Part2: {:?}", part_2);
}

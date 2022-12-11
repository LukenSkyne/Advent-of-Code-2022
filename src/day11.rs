use std::fs;
use regex::Regex;
use std::ops::{Add, Mul};

struct Monkey {
	items: Vec<i64>,
	operation: fn(i64, i64) -> i64,
	operand: i64,
	test: i64,
	res_a: usize,
	res_b: usize,
	inspection_count: i64,
}

impl Clone for Monkey {
	fn clone(&self) -> Self {
		Monkey {
			items: self.items.clone(),
			operation: self.operation,
			operand: self.operand,
			test: self.test,
			res_a: self.res_a,
			res_b: self.res_b,
			inspection_count: self.inspection_count,
		}
	}
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, worry_fn: fn(i64, i64) -> i64) -> i64 {
	let lcm = monkeys.iter()
		.map(|m| m.test)
		.reduce(|a, b| {
			return a * b
		})
		.unwrap();

	for _ in 0..rounds {
		for monkey_index in 0..monkeys.len() {
			for item_index in 0..monkeys[monkey_index].items.len() {
				let mut worry = monkeys[monkey_index].items[item_index];

				worry = (monkeys[monkey_index].operation)(worry, monkeys[monkey_index].operand);
				worry = worry_fn(worry, lcm);

				monkeys[monkey_index].inspection_count += 1;

				let target_monkey = if worry % monkeys[monkey_index].test == 0 {
					monkeys[monkey_index].res_a
				} else {
					monkeys[monkey_index].res_b
				};

				monkeys[target_monkey].items.push(worry);
			}

			monkeys[monkey_index].items.clear();
		}
	}

	let mut bla: Vec<_> = monkeys.iter().map(|m| m.inspection_count).collect();
	bla.sort();
	bla.reverse();

	return bla[0] * bla[1];
}

fn main() {
	println!("Day11");

	let input = fs::read_to_string("./input/day11.txt").unwrap();

	let re_nums = Regex::new(r"\d+").unwrap();
	let re_ops = Regex::new(r"old\s+([+*])\s+(\d+)?").unwrap();

	let monkeys = input.split("\n\n")
		.map(|monkey_str| {
			let monkey_props: Vec<&str> = monkey_str.split("\n").collect();

			let items: Vec<_> = re_nums.captures_iter(monkey_props[1])
				.map(|cap| cap[0].parse().unwrap())
				.collect();

			let ops = re_ops.captures(monkey_props[2]).unwrap();
			let operator = &ops[1];
			let operand_str = ops.get(2).map_or("", |m| m.as_str());
			let operand: i64 = operand_str.parse().unwrap_or(0);

			let operation: fn(i64, i64) -> i64 = if operand_str.is_empty() {
				|a, _| a.pow(2)
			} else {
				match operator {
					"*" => |a, b| a.mul(b),
					_ => |a, b| a.add(b),
				}
			};

			let test_cap = re_nums.captures(monkey_props[3]).unwrap();
			let test = (&test_cap[0]).parse().unwrap();

			let res_a_cap = re_nums.captures(monkey_props[4]).unwrap();
			let res_a = (&res_a_cap[0]).parse().unwrap();

			let res_b_cap = re_nums.captures(monkey_props[5]).unwrap();
			let res_b = (&res_b_cap[0]).parse().unwrap();

			return Monkey {
				items,
				operation,
				operand,
				test,
				res_a,
				res_b,
				inspection_count: 0,
			}
		})
		.collect::<Vec<_>>();

	let monkeys_1 = monkeys.to_vec();
	let monkeys_2 = monkeys.to_vec();

	let part_1 = simulate(monkeys_1, 20, |worry, _| worry / 3);
	println!("Part1: {:?}", part_1);

	let part_2 = simulate(monkeys_2, 10_000, |worry, lcm| worry % lcm);
	println!("Part2: {:?}", part_2);
}

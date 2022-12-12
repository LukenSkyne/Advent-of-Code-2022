use std::fs;
use std::cmp::{max, min};
use pathfinding::prelude::bfs;

fn find_char_pos_2d(maze: &Vec<Vec<char>>, chr: char) -> Option<(usize, usize)> {
	let mut x = None;
	let y = maze.iter()
		.position(|row| {
			x = row.iter()
				.position(|col| {
					*col == chr
				});

			return x.is_some();
		});

	if x.is_none() || y.is_none() {
		return None;
	}

	Some((x.unwrap(), y.unwrap()))
}

fn main() {
	println!("Day12");

	let input = fs::read_to_string("./input/day12.txt").unwrap();

	let mut maze = input.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| line.chars()
			.collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let start = find_char_pos_2d(&maze, 'S').unwrap();
	let end = find_char_pos_2d(&maze, 'E').unwrap();

	maze[start.1][start.0] = 'a';
	maze[end.1][end.0] = 'z';

	let get_successors = |(c_x, c_y): (usize, usize)| {
		let current = maze[c_y][c_x];
		let mut successors = vec![];

		let min_x = max(c_x as i32 - 1, 0) as usize;
		let min_y = max(c_y as i32 - 1, 0) as usize;
		let max_x = min(maze[0].len() - 1, c_x + 1) as usize;
		let max_y = min(maze.len() - 1, c_y + 1) as usize;

		for y in min_y..max_y+1 {
			for x in min_x..max_x+1 {
				let valid = (maze[y][x] as usize) < (current as usize) + 2;

				if valid && (x == c_x || y == c_y) {
					successors.push((x, y))
				}
			}
		}

		successors
	};

	let result = bfs(
		&start,
		|p| get_successors(*p),
		|p| *p == end
	).unwrap();

	let part_1 = result.len() - 1;
	println!("Part1: {:?}", part_1);

	let mut part_2_starts = vec![];

	for y in 0..maze.len() {
		for x in 0..maze[y].len() {
			if maze[y][x] == 'a' {
				part_2_starts.push((x, y));
			}
		}
	}

	let mut part_2_paths = vec![];

	for sta in part_2_starts {
		let res = bfs(
			&sta,
			|p| get_successors(*p),
			|p| *p == end
		);

		if res.is_none() {
			continue
		}

		part_2_paths.push(res.unwrap().len() - 1);
	}

	part_2_paths.sort();

	let part_2 = part_2_paths.first().unwrap();
	println!("Part2: {:?}", part_2);
}

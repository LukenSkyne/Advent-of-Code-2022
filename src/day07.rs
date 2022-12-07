use std::fs;

enum NodeType {
	Dir,
	File,
}

struct Node {
	name: String,
	size: usize,
	node_type: NodeType,

	index: usize,
	parent_index: Option<usize>,
	children_indices: Vec<usize>,
}

fn calc_size(arena: &Vec<Node>, index: usize) -> usize {
	if matches!(arena[index].node_type, NodeType::File) {
		return arena[index].size;
	}

	return arena[index].children_indices.iter().map(|&i| calc_size(arena, i)).sum::<usize>();
}

fn main() {
	println!("Day07");

	let input = fs::read_to_string("./input/day07.txt").unwrap();

	let mut arena = vec![Node {
		name: String::from("/"),
		size: 0,
		node_type: NodeType::Dir,
		index: 0,
		parent_index: None,
		children_indices: vec![],
	}];
	let mut current_index = 0;

	input.split("\n")
		.filter(|line| !line.is_empty())
		.for_each(|line| {
			let tokens = line.split(" ").collect::<Vec<_>>();

			if tokens.len() < 2 {
				return;
			}

			if tokens[0] == "$" {
				if tokens[1] == "cd" && tokens.len() > 2 {
					if tokens[2] == ".." {
						if arena[current_index].parent_index.is_some() {
							current_index = arena[current_index].parent_index.unwrap();
						}
					} else {
						let new_index = arena[current_index].children_indices.iter().find(|&&x| arena[x].name == tokens[2]);

						if new_index.is_some() {
							current_index = *new_index.unwrap()
						}
					}
				}
			} else {
				let is_dir = tokens[0] == "dir";
				let size = if is_dir { 0 } else { tokens[0].parse::<usize>().unwrap() };

				let new_index = arena.len();
				arena.push(Node {
					name: String::from(tokens[1]),
					size,
					node_type: if is_dir { NodeType::Dir } else { NodeType::File },
					index: new_index,
					parent_index: Some(current_index),
					children_indices: vec![],
				});
				arena[current_index].children_indices.push(new_index);
			}
		});

	let part_1 = arena.iter()
		.filter(|item| matches!(item.node_type, NodeType::Dir))
		.map(|item| calc_size(&arena, item.index))
		.filter(|&size| size <= 100_000)
		.sum::<usize>();

	println!("Part1: {:?}", part_1);

	let total_space = 70_000_000;
	let used_space = calc_size(&arena, 0);
	let need_space = 30_000_000 - (total_space - used_space);

	let mut part_2 = arena.iter()
		.filter(|item| matches!(item.node_type, NodeType::Dir))
		.map(|item| calc_size(&arena, item.index))
		.filter(|&size| size >= need_space)
		.collect::<Vec<_>>();

	part_2.sort();

	println!("Part2: {:?}", part_2.first().unwrap());
}

use std::fs;
use std::collections::VecDeque;

fn main() {
    println!("Day05");

    let input = fs::read_to_string("./input/day05.txt").unwrap();

    let (stack_data, move_data) = input.split_once("\n\n").unwrap();
    let mut supplies = Vec::<VecDeque<char>>::new();

    stack_data.split("\n").for_each(|layer| {
        layer.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .enumerate()
            .for_each(|(i, batch)| {
                if i + 1 > supplies.len() {
                    supplies.push(VecDeque::<char>::new());
                }

                if batch[1].is_alphabetic() {
                    supplies[i].push_front(batch[1]);
                }
            });
    });

    let moves = move_data.split("\n").filter(|line| !line.is_empty()).map(|line| {
        let tmp = line.replace("move ", "");
        let (amount, pos) = tmp.split_once(" from ").unwrap();
        let (from, to) = pos.split_once(" to ").unwrap();

        (
            amount.parse::<usize>().unwrap(),
            from.parse::<usize>().unwrap() - 1,
            to.parse::<usize>().unwrap() - 1,
        )
    }).collect::<Vec<_>>();

    let mut part_1 = supplies.clone();

    moves.iter().for_each(|&(amount, from, to)| {
        for _ in 0..amount {
            let tmp = part_1[from].pop_back().unwrap();
            part_1[to].push_back(tmp);
        }
    });

    print!("Part1: ");
    part_1.iter().for_each(|stack| {
        print!("{}", stack.back().unwrap());
    });
    println!();

    let mut part_2 = supplies.clone();

    moves.iter().for_each(|&(amount, from, to)| {
        let mut cache = VecDeque::<char>::new();

        for _ in 0..amount {
            let tmp = part_2[from].pop_back().unwrap();
            cache.push_front(tmp);
        }

        for elem in cache {
            part_2[to].push_back(elem);
        }
    });

    print!("Part2: ");
    part_2.iter().for_each(|stack| {
        print!("{}", stack.back().unwrap());
    });
    println!();
}

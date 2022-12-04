use std::fs;
use regex::Regex;

fn main() {
    println!("Day04");

    let input = fs::read_to_string("./input/day04.txt").unwrap();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let part_1 = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| match re.captures(line) {
            Some(caps) => {
                let extract = |index: usize| -> i32 { caps.get(index).unwrap().as_str().parse::<i32>().unwrap() };
                Some((
                    extract(1) ..= extract(2),
                    extract(3) ..= extract(4)
                ))
            }
            None => None,
        }.unwrap())
        .map(|(left, right)| {
            if (left.contains(&right.start()) && left.contains(&right.end())) ||
                (right.contains(&left.start()) && right.contains(&left.end())) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("Part1: {:?}", part_1);

    let part_2 = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| match re.captures(line) {
            Some(caps) => {
                let extract = |index: usize| -> i32 { caps.get(index).unwrap().as_str().parse::<i32>().unwrap() };
                Some((
                    extract(1) ..= extract(2),
                    extract(3) ..= extract(4)
                ))
            }
            None => None,
        }.unwrap())
        .map(|(left, right)| {
            if (left.contains(&right.start()) || left.contains(&right.end())) ||
                (right.contains(&left.start()) || right.contains(&left.end())) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("Part2: {:?}", part_2);
}

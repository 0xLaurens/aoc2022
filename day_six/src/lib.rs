use itertools::Itertools;
use regex::Regex;

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::with_capacity(9);
    input.lines().rev().skip(1).for_each(|line| {
        line.chars().skip(1).step_by(4).enumerate()
            .filter(|(_, item)| !item.is_ascii_whitespace())
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec = Vec::with_capacity(10);
                    new_vec.push(item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(item);
                }
            })
    });

    stacks
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut nums = input
        .split_ascii_whitespace()
        .skip(1).step_by(2).map(|num| num.parse().unwrap()).chunks(3).into_iter()
        .filter();
    instructions
}


pub fn process_part_one(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\r").unwrap();
    let mut crates = parse_crates(crates);
    let moves = parse_moves(moves);

    for (count, from, to) in moves {
        for _ in 0..count {
            let item = crates[from - 1].pop().unwrap();
            crates[to - 1].push(item);
        }
    }

    crates.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

// Part two
pub fn process_part_two(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\r").unwrap();
    let mut crates = parse_crates(crates);
    let moves = parse_moves(moves);

    for (count, from, to) in moves {
        let target_height = crates[to - 1].len();
        for _ in 0..count {
            let item = crates[from - 1].pop().unwrap();
            crates[to - 1].insert(target_height, item);
        }
    }
    crates.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT);
        assert_eq!(result, "CMZ");
    }

    // #[test]
    // fn part_two_test() {
    //     let result = process_part_two(INPUT);
    //     assert_eq!(result, 70); // }
}

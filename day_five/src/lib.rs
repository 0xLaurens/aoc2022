use itertools::Itertools;

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
    input
        .split_ascii_whitespace()
        .skip(1).step_by(2).filter_map(|num| num.parse().ok())
        .collect::<Vec<usize>>()
        .chunks(3)
        .map(|x| (x[0], x[1], x[2])).collect::<Vec<(_,_,_)>>()
}


pub fn process_part_one(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\r").unwrap();
    let mut crates = parse_crates(crates);
    let moves = parse_moves(moves);

    println!("{:?}", moves);

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

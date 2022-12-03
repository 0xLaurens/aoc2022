use std::collections::HashSet;
// Both parts
fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 96
    } else {
        c as i32 - 38
    }
}

// Part one
pub fn process_part_one(input: &str) -> i32 {
    let result: i32 = input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(lhs, rhs)|(lhs.chars().collect::<HashSet<_>>(), rhs.chars().collect::<HashSet<_>>()))
        .map(|(lhs, rhs)| *lhs.intersection(&rhs).next().unwrap())
        .map(|intersection| priority(intersection))
        .sum();
    result
}
// Part two
pub fn process_part_two(input: &str) -> i32 {
    let result: i32 = input
        .lines()
        .collect::<Vec<_>>().chunks(3)
        .map(|chunks| { ((chunks[0]), (chunks[1]), (chunks[2])) })
        .map(|(lhs, mid, rhs)| lhs.chars().find(|&i| mid.contains(i) && rhs.contains(i)).unwrap())
        .map(|intersection| priority(intersection))
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT_PART_ONE);
        assert_eq!(result, 157);
    }

    const INPUT_PART_TWO: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";

    #[test]
    fn part_two_test() {
        let result = process_part_two(INPUT_PART_TWO);
        assert_eq!(result, 70);
    }
}

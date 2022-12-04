use std::collections::HashSet;

// Both parts
fn priority(c: char) -> i32 {
    if c.is_lowercase() { return c as i32 - 96; }
    c as i32 - 38
}

// Part one
pub fn process_part_one(input: &str) -> i32 {
    let result: i32 = input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(lhs, rhs)| (lhs.chars().collect::<HashSet<_>>(), rhs.chars().collect::<HashSet<_>>()))
        .map(|(lhs, rhs)| *lhs.intersection(&rhs).next().unwrap())
        .map(|intersection| priority(intersection))
        .sum();
    result
}

// Part two
pub fn process_part_two(input: &str) -> i32 {
    input.lines().collect::<Vec<&str>>().chunks(3)
        .map(|x| priority(x[0].chars().find(|&i| x[1].contains(i) && x[2].contains(i)).unwrap())).sum::<i32>()
}

mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_two_test() {
        let result = process_part_two(INPUT);
        assert_eq!(result, 70);
    }
}

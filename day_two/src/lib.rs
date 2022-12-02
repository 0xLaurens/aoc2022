pub fn process_part_one(input: &str) -> i32 {
    let result = input
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .fold(0, |points, (elf, me)| match (elf, me) {
            ("A", "X") => points + (3 + 1),
            ("A", "Y") => points + (6 + 2),
            ("A", "Z") => points + (0 + 3),

            ("B", "X") => points + (0 + 1),
            ("B", "Y") => points + (3 + 2),
            ("B", "Z") => points + (6 + 3),

            ("C", "X") => points + (6 + 1),
            ("C", "Y") => points + (0 + 2),
            ("C", "Z") => points + (3 + 3),
            _ => 0
        });
    result
}

pub fn process_part_two(input: &str) -> i32 {
    let result = input
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .fold(0, |points, (elf, me)| match (elf, me) {
            ("A", "X") => points + (0 + 3),
            ("A", "Y") => points + (3 + 1),
            ("A", "Z") => points + (6 + 2),

            ("B", "X") => points + (0 + 1),
            ("B", "Y") => points + (3 + 2),
            ("B", "Z") => points + (6 + 3),

            ("C", "X") => points + (0 + 2),
            ("C", "Y") => points + (3 + 3),
            ("C", "Z") => points + (1 + 6),
            _ => points
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_two_test() {
        let result = process_part_two(INPUT);
        assert_eq!(result, 12);
    }
}

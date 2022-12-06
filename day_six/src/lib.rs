use std::collections::HashSet;

fn parse_packet(input: &str, pack_size: usize) -> usize {
    input.as_bytes().windows(pack_size).enumerate()
        .find_map(|(i, w)| {
            if w.iter().copied().collect::<HashSet<u8>>().len() == pack_size {
                Some(i + pack_size)
            } else { None }
        }).unwrap()
}

pub fn process_part_one(input: &str) -> usize {
    parse_packet(input, 4)
}

pub fn process_part_two(input: &str) -> usize {
    parse_packet(input, 14)
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_two_test() {
        let result = process_part_two(INPUT);
        assert_eq!(result, 19);
    }
}

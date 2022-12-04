pub fn process_part_one(input: &str) -> usize {
    input.lines()
        .map(|x| { x.split([',', '-']).map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>() })
        .filter(|x| (x[0] >= x[2] && x[1] <= x[3]) || (x[2] >= x[0] && x[3] <= x[1])).count()
}

pub fn process_part_two(input: &str) -> usize {
    input.lines()
        .map(|x| { x.split([',', '-']).map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>() })
        .filter(|x| x[0] <= x[3] && x[2] <= x[1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part_one_test() {
        let result = process_part_one(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two_test() {
        let result = process_part_two(INPUT);
        assert_eq!(result, 4);
    }
}

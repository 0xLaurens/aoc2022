pub fn process_part_one(input: &str) -> i32 {
    input.lines()
        .map(|x| {
            x.split(",")
                .flat_map(|range| range.split("-"))
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|vec| {
            (vec[0] >= vec[2] && vec[1] <= vec[3]) || (vec[2] >= vec[0] && vec[3] <= vec[1])
        })
        .count() as i32
}

pub fn process_part_two(input: &str) -> i32 {
    input.lines()
        .map(|x| {
            x.split(",")
                .flat_map(|range| range.split("-"))
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|vec| {
            vec[0] <= vec[3] && vec[2] <= vec[1]
        })
        .count() as i32
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

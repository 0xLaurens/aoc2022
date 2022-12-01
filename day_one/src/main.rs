#[bench]
fn main() {
    let input = include_str!("input.txt");
    let mut snacks= input
        .split("\r\n\r")
        .map(|x| x.lines()
            .filter_map(|x| x.parse::<i32>().ok()).sum::<i32>())
        .collect::<Vec<i32>>();

    snacks.sort();
    snacks.reverse();

    let max = snacks.first().unwrap();
    println!("Part 1: {:?}", max);
    let top3: i32 = snacks.iter().take(3).sum();
    println!("Part 2: {:?}", top3);
}

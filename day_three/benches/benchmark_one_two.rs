use day_three::process_part_one;
use day_three::process_part_two;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

// fn part_one_bench(c: &mut Criterion) {
//     let input = black_box(INPUT);
//     c.bench_function(
//         "part one",
//         |b| b.iter(|| process_part_one(input))
//     );
// }

fn part_two_bench(c: &mut Criterion) {
    let input = black_box(INPUT);
    c.bench_function(
        "part two",
        |b| b.iter(|| process_part_two(input.to_string()))
    );
}

// criterion_group!(benches, part_one_bench);
criterion_group!(benches, part_two_bench);
criterion_main!(benches);

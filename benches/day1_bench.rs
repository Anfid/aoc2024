use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc2024::day1;

const INPUT_REAL: &'static str = include_str!("../assets/day1.txt");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1_part1", |b| {
        b.iter(|| day1::part1(black_box(INPUT_REAL)))
    });
    c.bench_function("day1_part2", |b| {
        b.iter(|| day1::part2(black_box(INPUT_REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

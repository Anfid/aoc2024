use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc2024::day4;

const INPUT_REAL: &'static str = include_str!("../input/2024/day4.txt");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day4_part1", |b| {
        b.iter(|| day4::part1(black_box(INPUT_REAL)))
    });
    c.bench_function("day4_part2", |b| {
        b.iter(|| day4::part2(black_box(INPUT_REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"]
mod main;

fn bench_main(c: &mut Criterion) {
    c.bench_function("part 1 (sample)", |b| {
        let input = main::parse_input("input2.txt").unwrap();
        b.iter(|| main::part1(black_box(&input)))
    });

    c.bench_function("part 2 (sample)", |b| {
        let input = main::parse_input("input2.txt").unwrap();
        b.iter(|| main::part2(black_box(&input)))
    });

    c.bench_function("part 1 (real)", |b| {
        let input = main::parse_input("input.txt").unwrap();
        b.iter(|| main::part1(black_box(&input)))
    });

    c.bench_function("part 2 (real)", |b| {
        let input = main::parse_input("input.txt").unwrap();
        b.iter(|| main::part2(black_box(&input)))
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
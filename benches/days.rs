use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_day {
        ($day:tt) => {{
            const INPUT: &str = include_str!(concat!(
                std::env!("AOC_CACHE"),
                "/2025_",
                stringify!($day),
                ".txt"
            ));
            c.bench_function(stringify!($day), |b| {
                b.iter(|| advent_of_code_2025::days::$day::solve(black_box(INPUT)))
            });
            const INPUT_TEST: &str =
                include_str!(concat!("../test_input/", stringify!($day), ".txt"));
            c.bench_function(concat!(stringify!($day), " test"), |b| {
                b.iter(|| advent_of_code_2025::days::$day::solve(black_box(INPUT_TEST)))
            });
        }};
        ($day:tt, is_test) => {{
            const INPUT: &str = include_str!(concat!(
                std::env!("AOC_CACHE"),
                "/2025_",
                stringify!($day),
                ".txt"
            ));
            c.bench_function(stringify!($day), |b| {
                b.iter(|| advent_of_code_2025::days::$day::solve(black_box(INPUT), false))
            });
            const INPUT_TEST: &str =
                include_str!(concat!("../test_input/", stringify!($day), ".txt"));
            c.bench_function(concat!(stringify!($day), " test"), |b| {
                b.iter(|| advent_of_code_2025::days::$day::solve(black_box(INPUT_TEST), true))
            });
        }};
    }

    // bench_day!(day01);
    // bench_day!(day02);
    bench_day!(day03, is_test);
    // bench_day!(day04);
    // bench_day!(day05);
    // bench_day!(day06);
    // bench_day!(day07);
    // bench_day!(day08);
    // bench_day!(day09);
    // bench_day!(day10);
    // bench_day!(day11);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use advent_of_code_2024::{Solution, day1::Day1, day2::Day2, day3::Day3, day5::Day5}; //day6::Day6

// fn benchmark_day1(c: &mut Criterion) {
//     let input = fs::read_to_string("./inputs/day1.txt").unwrap();
//     let solution = Day1::parse(&input);
    
//     let mut group = c.benchmark_group("day1");
//     group.bench_function("parse", |b| {
//         b.iter(|| Day1::parse(black_box(&input)))
//     });
//     group.bench_function("part1", |b| {
//         b.iter(|| solution.part1())
//     });
//     group.bench_function("part2", |b| {
//         b.iter(|| solution.part2())
//     });
//     group.finish();
// }

// fn benchmark_day2(c: &mut Criterion) {
//     let input = fs::read_to_string("./inputs/day2.txt").unwrap();
//     let solution = Day2::parse(&input);
    
//     let mut group = c.benchmark_group("day2");
//     group.bench_function("parse", |b| {
//         b.iter(|| Day2::parse(black_box(&input)))
//     });
//     group.bench_function("part1", |b| {
//         b.iter(|| solution.part1())
//     });
//     group.bench_function("part2", |b| {
//         b.iter(|| solution.part2())
//     });
//     group.finish();
// }

// fn benchmark_day3(c: &mut Criterion) {
//     let input = fs::read_to_string("./inputs/day3.txt").unwrap();
//     let solution = Day3::parse(&input);
    
//     let mut group = c.benchmark_group("day3");
//     group.bench_function("parse", |b| {
//         b.iter(|| Day3::parse(black_box(&input)))
//     });
//     group.bench_function("part1", |b| {
//         b.iter(|| solution.part1())
//     });
//     group.bench_function("part2", |b| {
//         b.iter(|| solution.part2())
//     });
//     group.finish();
// }

fn benchmark_day5(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    let solution = Day5::parse(&input);
    
    let mut group = c.benchmark_group("day5");
    group.bench_function("parse", |b| {
        b.iter(|| Day5::parse(black_box(&input)))
    });
    group.bench_function("part1", |b| {
        b.iter(|| solution.part1())
    });
    group.bench_function("part2", |b| {
        b.iter(|| solution.part2())
    });
    group.finish();
}

// fn benchmark_day6(c: &mut Criterion) {
//     let input = fs::read_to_string("./inputs/day6.txt").unwrap();
//     let solution = Day6::parse(&input);
    
//     let mut group = c.benchmark_group("day6");
//     group.bench_function("parse", |b| {
//         b.iter(|| Day5::parse(black_box(&input)))
//     });
//     group.bench_function("part1", |b| {
//         b.iter(|| solution.part1())
//     });
//     group.bench_function("part2", |b| {
//         b.iter(|| solution.part2())
//     });
//     group.finish();
// }


criterion_group!(benches, benchmark_day5); //  benchmark_day1, benchmark_day2, benchmark_day3, 
criterion_main!(benches); 
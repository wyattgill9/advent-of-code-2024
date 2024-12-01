use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let part1_start = Instant::now();
    let part1_result = part1(&input);
    let part1_time = part1_start.elapsed();


    let part2_start = Instant::now();
    let part2_result = part2(&input);
    let part2_time = part2_start.elapsed();

    let total_time = total_start.elapsed();

    println!("\nResults:");
    println!("Part 1 (distance): {}", part1_result);
    println!("Part 2 (similarity): {}", part2_result);

    println!("\nTiming breakdown:");
    println!(
        "Part 1 time:    {:?} ({:.1}%)",
        part1_time,
        100.0 * part1_time.as_secs_f64() / total_time.as_secs_f64()
    );
    println!(
        "Part 2 time:    {:?} ({:.1}%)",
        part2_time,
        100.0 * part2_time.as_secs_f64() / total_time.as_secs_f64()
    );
    println!("Total time:     {:?}", total_time);
}

fn part1(input: &str) -> u64 {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input.lines().map(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
    }).unzip();
    left.sort();
    right.sort();
    left.into_iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> u64 {
    let (container1, container2): (Vec<u64>, Vec<u64>) = input.lines().map(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
    }).unzip();

    solution(container1, container2)
}

fn build_presence_map(source: Vec<u64>) -> HashMap<u64, u64> {
    let mut indexes = HashMap::<u64, u64>::new();

    source.into_iter().for_each(|curr| {
        *indexes.entry(curr).or_insert(0) += 1;
    });

    indexes
}

fn solution(container1: Vec<u64>, container2: Vec<u64>) -> u64 {
    let presence = build_presence_map(container2);

    container1.into_iter().fold(0, |acc, curr| {
        acc + curr * presence.get(&curr).cloned().unwrap_or(0)
    })
}
use std::time::Instant;
use rustc_hash::FxHashSet;

fn mean_and_stddev(data: &[std::time::Duration]) -> (std::time::Duration, std::time::Duration) {
    let sum: std::time::Duration = data.iter().sum();
    let mean = sum / data.len() as u32;
    
    let variance = data.iter()
        .map(|&x| {
            let diff = if x > mean { x - mean } else { mean - x };
            diff.as_nanos().pow(2)
        })
        .sum::<u128>() / data.len() as u128;
    
    let stddev = std::time::Duration::from_nanos((variance as f64).sqrt() as u64);
    (mean, stddev)
}

fn median(data: &[std::time::Duration]) -> std::time::Duration {
    let mut sorted = data.to_vec();
    sorted.sort();
    if sorted.len() % 2 == 0 {
        (sorted[sorted.len()/2 - 1] + sorted[sorted.len()/2]) / 2
    } else {
        sorted[sorted.len()/2]
    }
}

fn main() {
    const NUM_RUNS: u32 = 100_000;
    let mut read_times = Vec::with_capacity(NUM_RUNS as usize);
    let mut parse_times = Vec::with_capacity(NUM_RUNS as usize);
    let mut part1_times = Vec::with_capacity(NUM_RUNS as usize);
    let mut part2_times = Vec::with_capacity(NUM_RUNS as usize);
    
    let mut part1_result = 0;
    let mut part2_result = 0;

    for _ in 0..NUM_RUNS {
        let read_start = Instant::now();
        let input = std::fs::read_to_string("input.txt").unwrap();
        read_times.push(read_start.elapsed());

        let parse_start = Instant::now();
        let (_container1, _container2) = parse_input_fast(&input).unwrap();
        parse_times.push(parse_start.elapsed());

        let part1_start = Instant::now();
        part1_result = part1(&input);
        part1_times.push(part1_start.elapsed());

        let part2_start = Instant::now();
        part2_result = part2(&input);
        part2_times.push(part2_start.elapsed());
    }

    let median_read = median(&read_times);
    let median_parse = median(&parse_times);
    let median_part1 = median(&part1_times);
    let median_part2 = median(&part2_times);
    let total_median = median_read + median_parse + median_part1 + median_part2;

    let (mean_read, stddev_read) = mean_and_stddev(&read_times);
    let (mean_parse, stddev_parse) = mean_and_stddev(&parse_times);
    let (mean_part1, stddev_part1) = mean_and_stddev(&part1_times);
    let (mean_part2, stddev_part2) = mean_and_stddev(&part2_times);
    let total_mean = mean_read + mean_parse + mean_part1 + mean_part2;

    println!("\nResults for Unsafe implementation (over {} runs):", NUM_RUNS);
    println!("Part 1 (distance): {}", part1_result);
    println!("Part 2 (similarity): {}", part2_result);

    println!("\nMedian timing breakdown:");
    println!(
        "Read time:     {:?} ({:.1}%)",
        median_read,
        100.0 * median_read.as_secs_f64() / total_median.as_secs_f64()
    );
    println!(
        "Parse time:    {:?} ({:.1}%)",
        median_parse,
        100.0 * median_parse.as_secs_f64() / total_median.as_secs_f64()
    );
    println!(
        "Part 1 time:   {:?} ({:.1}%)",
        median_part1,
        100.0 * median_part1.as_secs_f64() / total_median.as_secs_f64()
    );
    println!(
        "Part 2 time:   {:?} ({:.1}%)",
        median_part2,
        100.0 * median_part2.as_secs_f64() / total_median.as_secs_f64()
    );
    println!("Total time:    {:?}", total_median);

    println!("\nStatistical timing breakdown (mean ± stddev):");
    println!(
        "Read time:     {:?} ± {:?} ({:.1}%)",
        mean_read,
        stddev_read,
        100.0 * mean_read.as_secs_f64() / total_mean.as_secs_f64()
    );
    println!(
        "Parse time:    {:?} ± {:?} ({:.1}%)",
        mean_parse,
        stddev_parse,
        100.0 * mean_parse.as_secs_f64() / total_mean.as_secs_f64()
    );
    println!(
        "Part 1 time:   {:?} ± {:?} ({:.1}%)",
        mean_part1,
        stddev_part1,
        100.0 * mean_part1.as_secs_f64() / total_mean.as_secs_f64()
    );
    println!(
        "Part 2 time:   {:?} ± {:?} ({:.1}%)",
        mean_part2,
        stddev_part2,
        100.0 * mean_part2.as_secs_f64() / total_mean.as_secs_f64()
    );
    println!("Total time:    {:?}", total_mean);
}

fn part1(input: &str) -> u64 {
    let (mut container1, mut container2) = parse_input_fast(input).unwrap();
    container1.sort_unstable();
    container2.sort_unstable();
    
    container1.into_iter()
        .zip(container2.into_iter())
        .map(|(a, b)| a.abs_diff(b) as u64)
        .sum()
}

fn part2(input: &str) -> u64 {
    let (container1, container2) = parse_input_fast(input).unwrap();
    let seen = container1.iter().copied().collect::<FxHashSet<_>>();
    container2.iter()
        .fold(0, |acc, &val| acc + val * seen.contains(&val) as u32) as u64
}

fn parse_input_fast(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    const LINE_LENGTH: usize = 14;

    let elements = input.len() / LINE_LENGTH;
    let mut x = Vec::with_capacity(elements);
    let mut y = Vec::with_capacity(elements);
    
    // Pre-allocate to avoid bounds checks during push
    unsafe {
        x.set_len(elements);
        y.set_len(elements);
    }

    for (idx, line) in input.as_bytes().chunks_exact(LINE_LENGTH).enumerate() {
        unsafe {
            let a0 = (*line.get_unchecked(0) - b'0') as u32 * 10_000;
            let a1 = (*line.get_unchecked(1) - b'0') as u32 * 1_000;
            let a2 = (*line.get_unchecked(2) - b'0') as u32 * 100;
            let a3 = (*line.get_unchecked(3) - b'0') as u32 * 10;
            let a4 = (*line.get_unchecked(4) - b'0') as u32 * 1;
            let a = a0 + a1 + a2 + a3 + a4;

            let b0 = (*line.get_unchecked(8) - b'0') as u32 * 10_000;
            let b1 = (*line.get_unchecked(9) - b'0') as u32 * 1_000;
            let b2 = (*line.get_unchecked(10) - b'0') as u32 * 100;
            let b3 = (*line.get_unchecked(11) - b'0') as u32 * 10;
            let b4 = (*line.get_unchecked(12) - b'0') as u32 * 1;
            let b = b0 + b1 + b2 + b3 + b4;

            *x.get_unchecked_mut(idx) = a;
            *y.get_unchecked_mut(idx) = b;
        }
    }

    Ok((x, y))
}
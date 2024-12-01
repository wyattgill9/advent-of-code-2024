use std::time::Instant;
use heapless::Vec as HeaplessVec;
use std::arch::x86_64::{
    _mm_set_epi16, _mm_sub_epi16, _mm_set1_epi16,
    _mm_madd_epi16
};
use std::mem;

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

fn part2(_input: &str) -> u64 {
    static DATA: &[u8] = include_bytes!("../../input.txt");
    
    let (container1, container2) = unsafe { parse_input_fast_static(DATA).unwrap() };
    let mut map = RobinHoodMap::<100000>::default();

    for &val in &container1 {
        map.insert(val);
    }
    
    container2.iter()
        .fold(0, |acc, &val| acc + val * map.get(val) as u32) as u64
}

struct RobinHoodMap<const N: usize> {
    keys: [u32; N],
    counts: [u8; N],
    used: [bool; N],
}

impl<const N: usize> Default for RobinHoodMap<N> {
    fn default() -> Self {
        Self {
            keys: [0; N],
            counts: [0; N],
            used: [false; N],
        }
    }
}

impl<const N: usize> RobinHoodMap<N> {
    #[inline]
    fn insert(&mut self, mut key: u32) {
        let mut idx = (key as usize) % N;
        let mut dist = 0;
        
        loop {
            if !self.used[idx] {
                self.keys[idx] = key;
                self.counts[idx] = 1;
                self.used[idx] = true;
                return;
            }
            if self.keys[idx] == key {
                self.counts[idx] += 1;
                return;
            }
            
            let existing_dist = ((idx + N - ((self.keys[idx] as usize) % N)) % N) as i32;
            if dist > existing_dist {
                let tmp_key = self.keys[idx];
                let tmp_count = self.counts[idx];
                self.keys[idx] = key;
                self.counts[idx] = 1;
                key = tmp_key;
                self.counts[idx] = tmp_count;
            }
            
            idx = (idx + 1) % N;
            dist += 1;
        }
    }

    #[inline]
    fn get(&self, key: u32) -> u8 {
        let mut idx = (key as usize) % N;
        let mut dist = 0;
        
        while self.used[idx] {
            if self.keys[idx] == key {
                return self.counts[idx];
            }
            let probe_dist = ((idx + N - ((self.keys[idx] as usize) % N)) % N) as i32;
            if dist > probe_dist {
                return 0;
            }
            idx = (idx + 1) % N;
            dist += 1;
        }
        0
    }
}

#[inline]
unsafe fn parse_input_fast_static(input: &[u8]) -> Result<(HeaplessVec<u32, 2000>, HeaplessVec<u32, 2000>), Box<dyn std::error::Error>> {
    const LINE_LENGTH: usize = 14;
    
    let elements = input.len() / LINE_LENGTH;
    if elements > 2000 {
        return Err("Input too large for HeaplessVec capacity".into());
    }
    
    let mut x = HeaplessVec::new();
    let mut y = HeaplessVec::new();
    
    for line in input.chunks_exact(LINE_LENGTH) {
        if x.extend_from_slice(&[parse_digits_simd(&line[0..5])]).is_err() {
            return Err("Failed to extend x vector".into());
        }
        if y.extend_from_slice(&[parse_digits_simd(&line[8..13])]).is_err() {
            return Err("Failed to extend y vector".into());
        }
    }

    Ok((x, y))
}

#[inline]
unsafe fn parse_digits_simd(digits: &[u8]) -> u32 {
    let digits = _mm_set_epi16(
        0, 0, 0,
        *digits.get_unchecked(4) as i16,
        *digits.get_unchecked(3) as i16,
        *digits.get_unchecked(2) as i16,
        *digits.get_unchecked(1) as i16,
        *digits.get_unchecked(0) as i16,
    );
    
    let nums = _mm_sub_epi16(digits, _mm_set1_epi16(b'0' as i16));
    
    let factors = _mm_set_epi16(0, 0, 0, 1, 10, 100, 1000, 10000);
    
    let mul: [u32; 4] = mem::transmute(_mm_madd_epi16(nums, factors));
    
    mul[0] + mul[1] + mul[2]
}

fn parse_input_fast(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    const LINE_LENGTH: usize = 14;
    let elements = input.len() / LINE_LENGTH;
    let mut x = Vec::with_capacity(elements);
    let mut y = Vec::with_capacity(elements);
    
    unsafe {
        x.set_len(elements);
        y.set_len(elements);
        
        for (idx, line) in input.as_bytes().chunks_exact(LINE_LENGTH).enumerate() {
            *x.get_unchecked_mut(idx) = parse_digits_simd(&line[0..5]);
            *y.get_unchecked_mut(idx) = parse_digits_simd(&line[8..13]);
        }
    }

    Ok((x, y))
}
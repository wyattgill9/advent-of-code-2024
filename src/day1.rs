use crate::Solution;
use rustc_hash::FxHashSet;

pub struct Day1 {
    container1: Vec<u32>,
    container2: Vec<u32>,
}

impl Solution for Day1 {
    fn parse(input: &str) -> Self {
        let (container1, container2) = parse_input(input);
        Self { container1, container2 }
    }

    fn part1(&self) -> u64 {
        let mut c1 = self.container1.clone();
        let mut c2 = self.container2.clone();
        c1.sort_unstable();
        c2.sort_unstable();
        
        c1.into_iter()
            .zip(c2.into_iter())
            .map(|(a, b)| a.abs_diff(b) as u64)
            .sum()
    }

    fn part2(&self) -> u64 {
        let seen = self.container1.iter().copied().collect::<FxHashSet<_>>();
        self.container2.iter()
            .fold(0, |acc, &val| acc + val * seen.contains(&val) as u32) as u64
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    const LINE_LENGTH: usize = 14;
    let elements = input.len() / LINE_LENGTH;
    let mut x = Vec::with_capacity(elements);
    let mut y = Vec::with_capacity(elements);

    for line in input.as_bytes().chunks_exact(LINE_LENGTH) {
        let a = parse_number(&line[0..5]);
        let b = parse_number(&line[8..13]);
        x.push(a);
        y.push(b);
    }

    (x, y)
}

#[inline]
fn parse_number(bytes: &[u8]) -> u32 {
    bytes.iter()
        .fold(0, |acc, &b| acc * 10 + (b - b'0') as u32)
}
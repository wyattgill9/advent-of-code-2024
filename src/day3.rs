use crate::Solution;
use regex::Regex;

pub struct Day3 {
    input: String,
    mul_re: Regex,
    do_re: Regex,
    dont_re: Regex,
}

impl Solution for Day3 {
    fn parse(input: &str) -> Self {
        Self {
            input: input.to_string(),
            mul_re: Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(),
            do_re: Regex::new(r"do\(\)").unwrap(),
            dont_re: Regex::new(r"don't\(\)").unwrap(),
        }
    }

    fn part1(&self) -> u64 {
        self.process_input(true) as u64
    }

    fn part2(&self) -> u64 {
        self.process_input(false) as u64
    }
}

impl Day3 {
    fn process_input(&self, start_enabled: bool) -> u32 {
        let mut total = 0;
        let mut enabled = start_enabled;
        let mut pos = 0;
        
        while pos < self.input.len() {
            if let Some(cap) = self.mul_re.find_at(&self.input, pos) {
                if enabled {
                    let caps = self.mul_re.captures(&self.input[cap.start()..cap.end()]).unwrap();
                    let x: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                    let y: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                    total += x * y;
                }
                pos = cap.end();
            } else if let Some(cap) = self.do_re.find_at(&self.input, pos) {
                enabled = true;
                pos = cap.end();
            } else if let Some(cap) = self.dont_re.find_at(&self.input, pos) {
                enabled = false;
                pos = cap.end();
            } else {
                pos += 1;
            }
        }
        
        total
    }
}

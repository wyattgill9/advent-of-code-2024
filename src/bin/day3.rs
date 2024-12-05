use std::fs;
use regex::Regex;

fn part1(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    let mut total = 0;
    let mut enabled = true;
    
    let mut pos = 0;
    while pos < input.len() {
        if let Some(cap) = mul_re.find_at(input, pos) {
            if enabled {
                let caps = mul_re.captures(&input[cap.start()..cap.end()]).unwrap();
                let x: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let y: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                total += x * y;
            }
            pos = cap.end();
        } else if let Some(cap) = do_re.find_at(input, pos) {
            enabled = true;
            pos = cap.end();
        } else if let Some(cap) = dont_re.find_at(input, pos) {
            enabled = false;
            pos = cap.end();
        } else {
            pos += 1;
        }
    }
    
    total
}

fn part2(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    let mut total = 0;
    let mut enabled = false;
    
    let mut pos = 0;
    while pos < input.len() {
        if let Some(cap) = mul_re.find_at(input, pos) {
            if enabled {
                let caps = mul_re.captures(&input[cap.start()..cap.end()]).unwrap();
                let x: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let y: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                total += x * y;
            }
            pos = cap.end();
        } else if let Some(cap) = do_re.find_at(input, pos) {
            enabled = true;
            pos = cap.end();
        } else if let Some(cap) = dont_re.find_at(input, pos) {
            enabled = false;
            pos = cap.end();
        } else {
            pos += 1;
        }
    }
    
    total
}

fn main() {
    let input = fs::read_to_string("src/inputs/day3.txt").expect("Failed to read input file");
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

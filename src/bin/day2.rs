#![allow(non_snake_case)]

use advent_of_code_2024::{Solution, day2::Day2};
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day2.txt").expect("Failed to read input file");
    let solution = Day2::parse(&input);
    
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}
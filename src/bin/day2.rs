use std::fs;

fn is_safe_report(report: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..report.len() {
        if report[i] > report[i - 1] {
            is_decreasing = false;
        } else if report[i] < report[i - 1] {
            is_increasing = false;
        }
    }

    if !is_increasing && !is_decreasing {
        return false;
    }

    for i in 1..report.len() {
        if !(1 <= (report[i] - report[i - 1]).abs() && (report[i] - report[i - 1]).abs() <= 3) {
            return false;
        }
    }

    true
}

fn can_be_safe_by_removing_one(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let modified_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &x)| x)
            .collect();
        
        if is_safe_report(&modified_report) {
            return true;
        }
    }
    false
}

fn part1(input_data: &[String]) -> i32 {
    let mut safe_count = 0;
    
    for line in input_data {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
            
        if is_safe_report(&report) {
            safe_count += 1;
        }
    }
    
    safe_count
}

fn part2(input_data: &[String]) -> i32 {
    let mut safe_count = 0;
    
    for line in input_data {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
            
        if is_safe_report(&report) || can_be_safe_by_removing_one(&report) {
            safe_count += 1;
        }
    }
    
    safe_count
}

fn main() {
    let input = fs::read_to_string("src/inputs/day2.txt").expect("Failed to read input file");
    let input_data: Vec<String> = input.lines().map(String::from).collect();
    
    println!("Part 1: {}", part1(&input_data));
    println!("Part 2: {}", part2(&input_data));
}
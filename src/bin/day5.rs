use std::collections::{HashMap, HashSet};
use std::fs;

fn read_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part1(data: &[String]) -> i32 {
    let mut rules = Vec::with_capacity(data.len() / 2);
    let mut lists = Vec::with_capacity(data.len() / 2);
    let mut reading_rules = true;

    for line in data {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let mut parts = line.split('|');
            rules.push((
                parts.next().unwrap().trim().parse::<i32>().unwrap(),
                parts.next().unwrap().trim().parse::<i32>().unwrap(),
            ));
        } else {
            lists.push(
                line.split(',')
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    let mut total = 0;
    for nums in &lists {
        let mut positions = vec![(0, 0); nums.len()];
        for (i, &x) in nums.iter().enumerate() {
            positions[i] = (x, i);
        }
        positions.sort_unstable();

        if rules.iter().all(|(a, b)| {
            let pos_a = positions.binary_search_by_key(a, |&(val, _)| val).ok();
            let pos_b = positions.binary_search_by_key(b, |&(val, _)| val).ok();
            match (pos_a, pos_b) {
                (Some(idx_a), Some(idx_b)) => positions[idx_a].1 < positions[idx_b].1,
                _ => true,
            }
        }) {
            total += nums[nums.len() / 2];
        }
    }
    total
}

fn part2(data: &[String]) -> i32 {
    let mut rules = Vec::with_capacity(data.len() / 2);
    let mut lists = Vec::with_capacity(data.len() / 2);
    let mut reading_rules = true;

    for line in data {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let mut parts = line.split('|');
            rules.push((
                parts.next().unwrap().trim().parse::<i32>().unwrap(),
                parts.next().unwrap().trim().parse::<i32>().unwrap(),
            ));
        } else {
            lists.push(
                line.split(',')
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    let mut total = 0;
    for nums in &lists {
        let mut positions = vec![(0, 0); nums.len()];
        for (i, &x) in nums.iter().enumerate() {
            positions[i] = (x, i);
        }
        positions.sort_unstable();

        if !rules.iter().all(|(a, b)| {
            let pos_a = positions.binary_search_by_key(a, |&(val, _)| val).ok();
            let pos_b = positions.binary_search_by_key(b, |&(val, _)| val).ok();
            match (pos_a, pos_b) {
                (Some(idx_a), Some(idx_b)) => positions[idx_a].1 < positions[idx_b].1,
                _ => true,
            }
        }) {
            let mut graph = vec![Vec::with_capacity(4); nums.len()];
            let mut value_to_index = vec![0; nums.len()];
            for (i, &x) in nums.iter().enumerate() {
                value_to_index[i] = x as usize;
            }
            
            for (a, b) in &rules {
                if let (Ok(idx_a), Ok(idx_b)) = (
                    positions.binary_search_by_key(a, |&(val, _)| val),
                    positions.binary_search_by_key(b, |&(val, _)| val)
                ) {
                    graph[idx_a].push(idx_b);
                }
            }

            let mut order = Vec::with_capacity(nums.len());
            let mut seen = vec![false; nums.len()];

            for i in 0..nums.len() {
                if !seen[i] {
                    visit2(i, &mut seen, &mut order, &graph);
                }
            }

            order.reverse();
            total += nums[order[order.len() / 2]];
        }
    }
    total
}

#[inline(always)]
fn visit2(
    n: usize,
    seen: &mut [bool],
    order: &mut Vec<usize>,
    graph: &[Vec<usize>],
) {
    if seen[n] {
        return;
    }
    seen[n] = true;
    for &next in &graph[n] {
        visit2(next, seen, order, graph);
    }
    order.push(n);
}

fn main() {
    let data = read_input("inputs/day5.txt");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

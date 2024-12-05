use crate::Solution;

pub struct Day2 {
    reports: Vec<Vec<i32>>
}

impl Solution for Day2 {
    fn parse(input: &str) -> Self {
        let reports = input.lines()
            .map(|line| line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect())
            .collect();
        Self { reports }
    }

    fn part1(&self) -> u64 {
        self.reports.iter()
            .filter(|report| is_safe_report(report))
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.reports.iter()
            .filter(|report| {
                is_safe_report(report) || can_be_safe_by_removing_one(report)
            })
            .count() as u64
    }
}

fn is_safe_report(report: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in report.windows(2) {
        match window[1].cmp(&window[0]) {
            std::cmp::Ordering::Greater => is_decreasing = false,
            std::cmp::Ordering::Less => is_increasing = false,
            _ => {}
        }
        
        if (window[1] - window[0]).abs() > 3 {
            return false;
        }
    }

    is_increasing || is_decreasing
}

fn can_be_safe_by_removing_one(report: &[i32]) -> bool {
    (0..report.len()).any(|i| {
        let modified: Vec<_> = report.iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &x)| x)
            .collect();
        is_safe_report(&modified)
    })
}
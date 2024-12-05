use crate::Solution;

pub struct Day5 {
    rules: Vec<(i32, i32)>,
    lists: Vec<Vec<i32>>
}

impl Solution for Day5 {
    fn parse(input: &str) -> Self {
        let mut rules = Vec::new();
        let mut lists = Vec::new();
        let mut reading_rules = true;
        
        for line in input.lines() {
            if line.is_empty() {
                reading_rules = false;
                continue;
            }
            
            if reading_rules {
                let mut parts = line.split('|');
                rules.push((
                    parts.next().unwrap().trim().parse().unwrap(),
                    parts.next().unwrap().trim().parse().unwrap()
                ));
            } else {
                lists.push(
                    line.split(',')
                        .map(|x| x.trim().parse().unwrap())
                        .collect()
                );
            }
        }
        
        Day5 { rules, lists }
    }

    fn part1(&self) -> u64 {
        self.lists.iter()
            .filter_map(|nums| {
                if self.rules.iter().all(|&(a, b)| {
                    match (nums.iter().position(|&x| x == a),
                          nums.iter().position(|&x| x == b)) {
                        (Some(i), Some(j)) => i < j,
                        _ => true
                    }
                }) {
                    Some(nums[nums.len() / 2] as u64)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(&self) -> u64 {
        let mut total = 0;
        
        for nums in &self.lists {
            if !self.rules.iter().all(|&(a, b)| {
                match (nums.iter().position(|&x| x == a),
                      nums.iter().position(|&x| x == b)) {
                    (Some(i), Some(j)) => i < j,
                    _ => true
                }
            }) {
                let mut graph = vec![vec![]; nums.len()];
                for &(a, b) in &self.rules {
                    if let (Some(i), Some(j)) = (
                        nums.iter().position(|&x| x == a),
                        nums.iter().position(|&x| x == b)
                    ) {
                        graph[i].push(j);
                    }
                }
                
                let mut seen = vec![false; nums.len()];
                let mut order = Vec::new();
                
                fn visit(n: usize, seen: &mut [bool], order: &mut Vec<usize>, 
                        graph: &[Vec<usize>]) {
                    if seen[n] { return; }
                    seen[n] = true;
                    for &next in &graph[n] {
                        visit(next, seen, order, graph);
                    }
                    order.push(n);
                }
                
                for i in 0..nums.len() {
                    if !seen[i] {
                        visit(i, &mut seen, &mut order, &graph);
                    }
                }
                
                order.reverse();
                total += nums[order[order.len() / 2]] as u64;
            }
        }
        
        total
    }
}

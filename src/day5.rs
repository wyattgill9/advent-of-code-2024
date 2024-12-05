use crate::Solution;

pub struct Day5 {
    rules: Vec<(i32, i32)>,
    lists: Vec<Vec<i32>>,
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
                let a: i32 = parts.next().unwrap().trim().parse().unwrap();
                let b: i32 = parts.next().unwrap().trim().parse().unwrap();
                rules.push((a, b));
            } else {
                let parsed_list: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
                lists.push(parsed_list);
            }
        }
        Day5 { rules, lists }
    }

    fn part1(&self) -> u64 {
        let mut sum = 0u64;
        for nums in &self.lists {
            let mut valid = true;
            for &(a, b) in &self.rules {
                unsafe {
                    let pos_a = Day5::fast_position(nums.as_ptr(), nums.len(), a);
                    let pos_b = Day5::fast_position(nums.as_ptr(), nums.len(), b);
                    if pos_a >= pos_b {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                sum += nums[nums.len() / 2] as u64;
            }
        }
        sum
    }

    fn part2(&self) -> u64 {
        let mut total = 0u64;
        for nums in &self.lists {
            let len = nums.len();
            unsafe {
                let mut graph: Vec<Vec<usize>> = vec![Vec::new(); len];
                for &(a, b) in &self.rules {
                    if let (Some(i), Some(j)) = (
                        Day5::fast_position(nums.as_ptr(), len, a),
                        Day5::fast_position(nums.as_ptr(), len, b),
                    ) {
                        graph[i].push(j);
                    }
                }
                let mut seen = vec![false; len];
                let mut order = Vec::with_capacity(len);
                for i in 0..len {
                    if !seen[i] {
                        Day5::visit(i, &mut seen, &mut order, &graph);
                    }
                }
                total += nums[*order.get_unchecked(len / 2)] as u64;
            }
        }
        total
    }
}

impl Day5 {
    #[target_feature(enable = "avx2")]
    unsafe fn fast_position(ptr: *const i32, len: usize, target: i32) -> Option<usize> {
        use std::arch::x86_64::*;

        let target_vec = _mm256_set1_epi32(target); 
        let chunks = len / 8; 
        let remainder = len % 8;

        for i in 0..chunks {
            let offset_ptr = ptr.add(i * 8);
            let vec = _mm256_loadu_si256(offset_ptr as *const __m256i); 
            let cmp = _mm256_cmpeq_epi32(vec, target_vec);
            let mask = _mm256_movemask_epi8(cmp); 

            if mask != 0 {
                let first_match = mask.trailing_zeros() / 4;
                return Some(i * 8 + first_match as usize);
            }
        }

        for i in (len - remainder)..len {
            if *ptr.add(i) == target {
                return Some(i);
            }
        }
        None
    }   

    unsafe fn visit(
        node: usize,
        seen: &mut [bool],
        order: &mut Vec<usize>,
        graph: &[Vec<usize>],
    ) {
        if *seen.get_unchecked(node) {
            return;
        }
        *seen.get_unchecked_mut(node) = true;
        for &next in graph.get_unchecked(node) {
            Day5::visit(next, seen, order, graph);
        }
        order.push(node);
    }
}

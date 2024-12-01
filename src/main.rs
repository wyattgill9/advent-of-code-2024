use std::io::BufRead;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();
    
    // Parsing phase
    let parse_start = Instant::now();
    let (nums1, nums2): (Vec<_>, Vec<_>) = std::fs::File::open("input.txt")
        .map(|f| std::io::BufReader::new(f).lines())
        .unwrap()
        .flatten()
        .map(|l| {
            let bytes = l.as_bytes();
            debug_assert!(bytes.len() == 13 || bytes.len() == 14, 
                "Line length must be 13 or 14 bytes, got {}", bytes.len());
            
            let mut n1 = 0u32;
            for &b in bytes.iter().take_while(|&&b| b != b' ') {
                if b.is_ascii_digit() {
                    n1 = n1 * 10 + (b - b'0') as u32;
                }
            }

            let mut n2 = 0u32;
            for &b in bytes.iter().skip_while(|&&b| b != b' ').skip(1) {
                if b.is_ascii_digit() {
                    n2 = n2 * 10 + (b - b'0') as u32;
                }
            }
            
            (n1, n2)
        })
        .unzip();
    let parse_time = parse_start.elapsed();
    
    // Distance calculation phase
    let dist_start = Instant::now();
    let (mut s1, mut s2) = (nums1.clone(), nums2.clone());
    s1.sort_unstable();
    s2.sort_unstable();
    let dist: u32 = s1.iter().zip(&s2)
        .map(|(&a, &b)| if a > b { a - b } else { b - a })
        .sum();
    let dist_time = dist_start.elapsed();
    
    let score_start = Instant::now();
    let mut counts = [0u32; 100_000];
    nums2.iter().for_each(|&n| counts[n as usize] += 1);
    let score: u64 = nums1.iter()
        .map(|&n| n as u64 * counts[n as usize] as u64)
        .sum();
    let score_time = score_start.elapsed();
    
    let total_time = total_start.elapsed();

    println!("\nResults:");
    println!("Total distance: {}", dist);
    println!("Similarity score: {}", score);
    
    println!("\nTiming breakdown:");
    println!("Parsing time:   {:?} ({:.1}%)", parse_time, 100.0 * parse_time.as_secs_f64() / total_time.as_secs_f64());
    println!("Distance time:  {:?} ({:.1}%)", dist_time, 100.0 * dist_time.as_secs_f64() / total_time.as_secs_f64());
    println!("Score time:     {:?} ({:.1}%)", score_time, 100.0 * score_time.as_secs_f64() / total_time.as_secs_f64());
    println!("Total time:     {:?}", total_time);
}
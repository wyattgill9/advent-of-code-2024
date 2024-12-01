use std::io::BufRead;

fn main() {
    let (nums1, nums2): (Vec<_>, Vec<_>) = std::fs::File::open("../input.txt")
        .map(|f| std::io::BufReader::new(f).lines())
        .unwrap()
        .flatten()
        .map(|l| {
            let bytes = l.as_bytes();
            let mut i = 0;
            let mut n1 = 0u32;

            while i < bytes.len() && bytes[i] != b' ' {
                if !bytes[i].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                n1 = n1 * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }

            i += 1;

            let mut n2 = 0u32;
            while i < bytes.len() {
                if !bytes[i].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                n2 = n2 * 10 + (bytes[i] - b'0') as u32;
                i += 1;
            }
            
            (n1, n2)
        })
        .unzip();

    // Part 1:
    let (mut s1, mut s2) = (nums1.clone(), nums2.clone());
    s1.sort_unstable();
    s2.sort_unstable();
    let dist: u32 = s1.iter().zip(&s2)
        .map(|(&a, &b)| if a > b { a - b } else { b - a })
        .sum();
    
    // Part 2:
    let mut counts = [0u32; 100_000];
    nums2.iter().for_each(|&n| counts[n as usize] += 1);
    let score: u64 = nums1.iter()
        .map(|&n| n as u64 * counts[n as usize] as u64)
        .sum();

    println!("Total distance: {}", dist);
    println!("Similarity score: {}", score);
}
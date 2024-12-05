use std::fs;

fn is_xmas_pattern(grid: &[Vec<char>], row: usize, col: usize, dx: i32, dy: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let pattern = "XMAS";
    
    for i in 0..4 {
        let new_row = row as i32 + i * dx;
        let new_col = col as i32 + i * dy;
        
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }
        
        if grid[new_row as usize][new_col as usize] != pattern.chars().nth(i).unwrap() {
            return false;
        }
    }
    
    true
}

fn part1(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [(0,1), (0,-1), (1,0), (-1,0), (1,1), (1,-1), (-1,1), (-1,-1)];
    
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if is_xmas_pattern(grid, i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if grid[i][j] != 'A' {
                continue;
            }
            
            let diag1 = (grid[i-1][j-1], grid[i+1][j+1]);
            let diag2 = (grid[i-1][j+1], grid[i+1][j-1]);
            
            if (diag1 == ('M', 'S') || diag1 == ('S', 'M')) &&
               (diag2 == ('M', 'S') || diag2 == ('S', 'M')) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("src/inputs/day4.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
